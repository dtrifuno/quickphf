use std::{env, fs, iter::repeat_with, path::Path};

use quickphf_codegen::{build_map, build_raw_map};

const SEED: u64 = 42;
static SIZES: &[usize] = &[
    5, 10, 100, 1000, 10_000, 50_000, 100_000, 250_000, 500_000, 750_000, 1_000_000,
];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("sizes.rs");
    fs::write(&dest_path, &format!("{:?}", SIZES)).unwrap();

    for (i, &size) in SIZES.iter().enumerate() {
        let mut rng = fastrand::Rng::with_seed(SEED);

        let keys: Vec<_> = repeat_with(|| rng.u64(..).wrapping_shl(16))
            .take(size)
            .collect();
        let values: Vec<_> = repeat_with(|| rng.u64(..)).take(size).collect();

        // Generate rust-phf static maps
        let mut phf_builder: phf_codegen::Map<u64> = phf_codegen::Map::new();
        for (k, v) in keys.iter().zip(values.iter()) {
            phf_builder.entry(*k, &format!("{}", v));
        }
        let phf_code = phf_builder.build().to_string();

        let file_name = format!("phf_map_{}.rs", i);
        let dest_path = Path::new(&out_dir).join(file_name);
        fs::write(&dest_path, phf_code).unwrap();

        // Generate quickphf static maps
        let quickphf_code = build_map(&keys, &values).to_string();

        let file_name = format!("quickphf_map_{}.rs", i);
        let dest_path = Path::new(&out_dir).join(file_name);
        fs::write(&dest_path, quickphf_code).unwrap();

        // Generate quickphf static raw maps
        let quickphf_code = build_raw_map(&keys, &values).to_string();

        let file_name = format!("quickphf_raw_map_{}.rs", i);
        let dest_path = Path::new(&out_dir).join(file_name);
        fs::write(&dest_path, quickphf_code).unwrap();
    }
}
