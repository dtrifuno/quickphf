use benchmarks::{PHF_MAPS, QUICKPHF_MAPS, QUICKPHF_RAW_MAPS, SIZES};

const BATCH_SIZE: usize = 1000;
const SEED: u64 = 42;

#[divan::bench(args = SIZES, sample_size = 1)]
fn phf_map(bencher: divan::Bencher, size: usize) {
    let mut rng = fastrand::Rng::with_seed(SEED)

    let index = SIZES.iter().position(|&s| s == size).unwrap();
    let map = &PHF_MAPS[index];
    let keys = map.keys().copied().collect::<Vec<_>>();

    bencher
        .with_inputs(|| (0..BATCH_SIZE).map(|_| keys[rng.usize(0..size)]))
        .bench_local_refs(|queries| {
            for query in queries {
                divan::black_box(map.get(&query).unwrap());
            }
        })
}

#[divan::bench(args = SIZES, sample_size = 1)]
fn quickphf_map(bencher: divan::Bencher, size: usize) {
    let mut rng = fastrand::Rng::with_seed(SEED)

    let index = SIZES.iter().position(|&s| s == size).unwrap();
    let map = &QUICKPHF_MAPS[index];
    let keys = map.keys().copied().collect::<Vec<_>>();

    bencher
        .with_inputs(|| (0..BATCH_SIZE).map(|_| keys[rng.usize(0..size)]))
        .bench_local_refs(|queries| {
            for query in queries {
                divan::black_box(map.get(&query).unwrap());
            }
        })
}

#[divan::bench(args = SIZES, sample_size = 1)]
fn quickphf_raw_map(bencher: divan::Bencher, size: usize) {
    let mut rng = fastrand::Rng::with_seed(SEED)

    let index = SIZES.iter().position(|&s| s == size).unwrap();
    let map = &QUICKPHF_RAW_MAPS[index];
    let keys = &QUICKPHF_MAPS[index].keys().copied().collect::<Vec<_>>();

    bencher
        .with_inputs(|| (0..BATCH_SIZE).map(|_| keys[rng.usize(0..size)]))
        .bench_local_refs(|queries| {
            for query in queries {
                divan::black_box(map.get(&query));
            }
        })
}

fn main() {
    divan::main();
}
