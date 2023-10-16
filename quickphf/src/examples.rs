pub static HOLIDAYS_PER_MONTH: crate::RawPhfMap<&'static str, i32> = crate::RawPhfMap::new(
    4294967296,
    &[0, 0, 1, 0, 4, 0, 1, 4, 11],
    &[0, 1, 1, 0, 1, 0, 1, 0, 1, 2, 1, 2],
    &[0],
);

pub static EMPTY_RAW_MAP: crate::RawPhfMap<&'static str, i32> =
    crate::RawPhfMap::new(0, &[0], &[], &[0]);

pub static FOURTH_POWERS_TO_ROOTS: crate::PhfMap<i32, i32> = crate::PhfMap::new(
    4294967296,
    &[0, 0, 0, 1, 1, 1, 2],
    &[
        (2401, 7),
        (625, 5),
        (1296, 6),
        (256, 4),
        (4096, 8),
        (81, 3),
        (6561, 9),
        (16, 2),
        (10000, 10),
        (1, 1),
    ],
    &[3],
);

pub static EMPTY_MAP: crate::PhfMap<&'static str, i32> = crate::PhfMap::new(0, &[0], &[], &[0]);

pub static DIGITS: crate::PhfSet<i32> = crate::PhfSet::new(
    4294967296,
    &[8, 0, 0, 9, 0, 0, 6],
    &[3, 9, 0, 8, 6, 5, 7, 1, 2, 4],
    &[0],
);

pub static EVEN_DIGITS: crate::PhfSet<i32> =
    crate::PhfSet::new(4294967296, &[0, 2, 0, 1, 0], &[8, 2, 6, 0, 4], &[1, 2]);

pub static PRIME_DIGITS: crate::PhfSet<i32> =
    crate::PhfSet::new(4294967296, &[2, 0, 0, 0], &[5, 7, 2, 3], &[2]);

pub static EMPTY_SET: crate::PhfSet<u64> = crate::PhfSet::new(0, &[0], &[], &[0]);
