//! Code for generating a PTHash-based perfect hash function.

use core::hash::Hash;

use quickdiv::DivisorU64;
use quickphf::shared::*;

const MAX_ALPHA: f64 = 0.99;
const MIN_C: f64 = 1.5;

fn ilog2(n: u64) -> u32 {
    63 - n.leading_zeros()
}

/// Parameters for a PTHash perfect hash function.
#[derive(Debug)]
pub struct Phf {
    pub seed: u64,
    pub pilots_table: Vec<u16>,
    pub map: Vec<u32>,
    pub free: Vec<u32>,
}

/// Generate a perfect hash function using PTHash for the given collection of keys.
pub fn generate_phf<H: Eq + Hash>(entries: &[H]) -> Phf {
    if entries.is_empty() {
        return Phf {
            seed: 0,
            map: vec![],
            // These vectors have to be non-empty so that the number of buckets and codomain
            // length are non-zero, and thus can be used to instantiate precomputed divisors.
            pilots_table: vec![0],
            free: vec![0],
        };
    }

    let n = entries.len() as u64;
    let lg = ilog2(n) as f64;
    let c = MIN_C + 0.2 * lg;
    let buckets_len = DivisorU64::new(if n > 1 {
        ((c * n as f64) / lg).ceil() as u64
    } else {
        1
    });

    let alpha = MAX_ALPHA - 0.001 * lg;
    let codomain_len = DivisorU64::new({
        let candidate = (n as f64 / alpha).ceil() as u64;
        candidate + (1 - candidate % 2)
    });

    (1..)
        .find_map(|n| try_generate_phf(entries, buckets_len, codomain_len, n << 32))
        .unwrap()
}

fn try_generate_phf<H: Eq + Hash>(
    entries: &[H],
    buckets_len: DivisorU64,
    codomain_len: DivisorU64,
    seed: u64,
) -> Option<Phf> {
    // We begin by hashing the entries, assigning them to buckets, and checking for collisions.
    struct HashedEntry {
        idx: usize,
        hash: u64,
        bucket: usize,
    }

    let mut hashed_entries: Vec<_> = entries
        .iter()
        .enumerate()
        .map(|(idx, entry)| {
            let hash = hash_key(entry, seed);
            let bucket = get_bucket(hash, buckets_len);

            HashedEntry { idx, hash, bucket }
        })
        .collect();

    hashed_entries.sort_unstable_by_key(|e| (e.bucket, e.hash));

    for window in hashed_entries.as_slice().windows(2) {
        let e0 = &window[0];
        let e1 = &window[1];

        if e0.hash == e1.hash && e0.bucket == e1.bucket {
            if entries[e0.idx] == entries[e1.idx] {
                panic!("duplicate keys at indices {} and {}", e0.idx, e1.idx);
            }
            return None;
        }
    }

    //
    struct BucketData {
        idx: usize,
        start_idx: usize,
        size: usize,
    }

    let mut buckets = Vec::with_capacity(buckets_len.get() as usize);

    let mut start_idx = 0;
    for idx in 0..buckets_len.get() as usize {
        let size = hashed_entries[start_idx..]
            .iter()
            .take_while(|entry| entry.bucket == idx)
            .count();

        buckets.push(BucketData {
            idx,
            start_idx,
            size,
        });
        start_idx += size;
    }

    buckets.sort_unstable_by(|b1, b2| b1.size.cmp(&b2.size).reverse());

    //
    let mut pilots_table = vec![0; buckets_len.get() as usize];

    // Using a sentinel value instead of an Option here allows us to avoid an expensive
    // reallocation. This is fine since the compiler cannot handle a static map with more than
    // a few million entries anyway.
    assert!((entries.len() as u64) < (u32::MAX as u64));
    const EMPTY: u32 = u32::MAX;
    let mut map = vec![EMPTY; codomain_len.get() as usize];

    let mut values_to_add = Vec::new();
    for bucket in buckets {
        let mut pilot_found = false;

        let bucket_start = bucket.start_idx;
        let bucket_end = bucket_start + bucket.size;
        let bucket_entries = &hashed_entries[bucket_start..bucket_end];

        'pilots: for pilot in 0u16..=u16::MAX {
            values_to_add.clear();
            let pilot_hash = hash_pilot_value(pilot);

            // Check for collisions with items from previous buckets.
            for entry in bucket_entries.iter() {
                let destination = get_index(entry.hash, pilot_hash, codomain_len);

                if map[destination as usize] != EMPTY {
                    continue 'pilots;
                }

                values_to_add.push((entry.idx, destination));
            }

            // Check for collisions within this bucket.
            values_to_add.sort_unstable_by_key(|k| k.1);
            for window in values_to_add.as_slice().windows(2) {
                if window[0].1 == window[1].1 {
                    continue 'pilots;
                }
            }

            pilot_found = true;
            for &(idx, destination) in &values_to_add {
                map[destination] = idx as u32;
            }
            pilots_table[bucket.idx] = pilot;
            break;
        }

        if !pilot_found {
            return None;
        }
    }

    // At this point `map` is a table of size `n_prime`, but with `n` values.
    // We need to move the items from the back into the empty slots at the
    // front, and compute the vector `free` that will point to their new locations.
    let extra_slots = codomain_len.get() as usize - entries.len();
    let mut free = vec![0; extra_slots];

    let mut back_idx = entries.len();
    for front_idx in 0..entries.len() {
        if map[front_idx] != EMPTY {
            continue;
        }

        while map[back_idx] == EMPTY {
            back_idx += 1;
        }

        map[front_idx] = map[back_idx];
        free[back_idx - entries.len()] = front_idx as u32;
        back_idx += 1;
    }

    map.truncate(entries.len());

    Some(Phf {
        seed,
        pilots_table,
        map,
        free,
    })
}
