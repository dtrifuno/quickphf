use std::iter::repeat_with;

use quickphf_codegen::phf::generate_phf;

use benchmarks::SIZES;

const SEED: u64 = 42;

#[divan::bench(args = SIZES, max_time = 10)]
fn quickphf(bencher: divan::Bencher, size: usize) {
    let mut rng = fastrand::Rng::with_seed(SEED);

    bencher
        .with_inputs(|| repeat_with(|| rng.u64(..)).take(size).collect())
        .bench_local_refs(|keys: &mut Vec<u64>| generate_phf(keys));
}

#[divan::bench(args = SIZES, max_time = 10)]
fn phf(bencher: divan::Bencher, size: usize) {
    let mut rng = fastrand::Rng::with_seed(SEED);

    bencher
        .with_inputs(|| repeat_with(|| rng.u64(..)).take(size).collect())
        .bench_local_refs(|keys: &mut Vec<u64>| phf_generator::generate_hash(keys));
}

fn main() {
    divan::main();
}
