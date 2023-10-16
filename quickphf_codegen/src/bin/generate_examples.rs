//! Generates the `examples` module for `quickphf`.

use std::io::Write as IOWrite;
use std::{fmt::Write, fs::File, io::BufWriter, path::Path};

use quickphf_codegen::{build_map, build_raw_map, build_set, CodeWriter};

const DESTINATION: &str = "examples.rs";

fn main() {
    let mut buffer = String::new();

    let months = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    ];
    let holidays = [2, 1, 0, 0, 0, 1, 1, 0, 1, 1, 2, 1];
    let holidays_per_month = build_raw_map(&months, &holidays);
    writeln!(
        &mut buffer,
        "pub static HOLIDAYS_PER_MONTH: crate::RawPhfMap<&'static str, i32> = {};\n",
        holidays_per_month
    )
    .unwrap();

    let empty_raw_map: CodeWriter<'_, &str, i32> = build_raw_map(&[], &[]);
    writeln!(
        &mut buffer,
        "pub static EMPTY_RAW_MAP: crate::RawPhfMap<&'static str, i32> = {};\n",
        empty_raw_map
    )
    .unwrap();

    let roots = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let fourth_powers = roots.map(|x| x * x * x * x);
    let powers_to_roots = build_map(&fourth_powers, &roots);
    writeln!(
        &mut buffer,
        "pub static FOURTH_POWERS_TO_ROOTS: crate::PhfMap<i32, i32> = {};\n",
        powers_to_roots
    )
    .unwrap();

    let empty_map: CodeWriter<'_, &str, i32> = build_map(&[], &[]);
    writeln!(
        &mut buffer,
        "pub static EMPTY_MAP: crate::PhfMap<&'static str, i32> = {};\n",
        empty_map
    )
    .unwrap();

    let digits_set = build_set(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    writeln!(
        &mut buffer,
        "pub static DIGITS: crate::PhfSet<i32> = {};\n",
        digits_set
    )
    .unwrap();

    let even_digits_set = build_set(&[0, 2, 4, 6, 8]);
    writeln!(
        &mut buffer,
        "pub static EVEN_DIGITS: crate::PhfSet<i32> = {};\n",
        even_digits_set
    )
    .unwrap();

    let prime_digits_set = build_set(&[2, 3, 5, 7]);
    writeln!(
        &mut buffer,
        "pub static PRIME_DIGITS: crate::PhfSet<i32> = {};\n",
        prime_digits_set
    )
    .unwrap();

    let empty_set: CodeWriter<'_, u64> = build_set(&[]);
    writeln!(
        &mut buffer,
        "pub static EMPTY_SET: crate::PhfSet<u64> = {};\n",
        empty_set
    )
    .unwrap();

    let buffer = buffer.replace("::quickphf::", "crate::");

    let path = Path::new(DESTINATION);
    let mut file = BufWriter::new(File::create(path).unwrap());
    write!(&mut file, "{}", buffer).unwrap();
}
