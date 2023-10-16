use std::{cell::RefCell, iter::repeat_with};

use quickphf_codegen::generator::generate_phf;

use benchmarks::SIZES;

const SEED: u64 = 42;

#[divan::bench(consts = SIZES, max_time = std::time::Duration::from_secs(10))]
fn quickphf<const S: usize>(bencher: divan::Bencher) {
    let rng = RefCell::new(fastrand::Rng::with_seed(SEED));

    bencher
        .with_inputs(|| repeat_with(|| rng.borrow_mut().u64(..)).take(S).collect())
        .bench_local_refs(|keys: &mut Vec<u64>| generate_phf(keys));
}

#[divan::bench(consts = SIZES, max_time = std::time::Duration::from_secs(10))]
fn phf<const S: usize>(bencher: divan::Bencher) {
    let rng = RefCell::new(fastrand::Rng::with_seed(SEED));

    bencher
        .with_inputs(|| repeat_with(|| rng.borrow_mut().u64(..)).take(S).collect())
        .bench_local_refs(|keys: &mut Vec<u64>| phf_generator::generate_hash(keys));
}

fn main() {
    divan::main();
}
