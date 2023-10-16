use core::hash::{Hash, Hasher};

#[inline]
pub fn hash_key<K: Hash + ?Sized>(key: &K, seed: u64) -> u64 {
    let mut hasher = wyhash::WyHash::with_seed(seed);
    key.hash(&mut hasher);
    hasher.finish()
}

#[inline]
pub fn hash_pilot_value(pilot_value: u16) -> u64 {
    /// Multiplicative constant from `fxhash`.
    const K: u64 = 0x517cc1b727220a95;
    (pilot_value as u64).wrapping_mul(K)
}

#[inline]
pub fn get_bucket(key_hash: u64, buckets: quickdiv::DivisorU64) -> usize {
    (key_hash % buckets) as usize
}

#[inline]
pub fn get_index(key_hash: u64, pilot_hash: u64, codomain_len: quickdiv::DivisorU64) -> usize {
    ((key_hash ^ pilot_hash) % codomain_len) as usize
}
