pub const SIZES: &[usize] = &include!(concat!(env!("OUT_DIR"), "/sizes.rs"));

pub static QUICKPHF_MAPS: [::quickphf::PhfMap<u64, u64>; 11] = [
    include!(concat!(env!("OUT_DIR"), "/quickphf_map_0.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_map_1.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_map_2.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_map_3.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_map_4.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_map_5.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_map_6.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_map_7.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_map_8.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_map_9.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_map_10.rs")),
];

pub static QUICKPHF_RAW_MAPS: [::quickphf::RawPhfMap<u64, u64>; 11] = [
    include!(concat!(env!("OUT_DIR"), "/quickphf_raw_map_0.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_raw_map_1.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_raw_map_2.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_raw_map_3.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_raw_map_4.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_raw_map_5.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_raw_map_6.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_raw_map_7.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_raw_map_8.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_raw_map_9.rs")),
    include!(concat!(env!("OUT_DIR"), "/quickphf_raw_map_10.rs")),
];

pub static PHF_MAPS: [::phf::Map<u64, u64>; 11] = [
    include!(concat!(env!("OUT_DIR"), "/phf_map_0.rs")),
    include!(concat!(env!("OUT_DIR"), "/phf_map_1.rs")),
    include!(concat!(env!("OUT_DIR"), "/phf_map_2.rs")),
    include!(concat!(env!("OUT_DIR"), "/phf_map_3.rs")),
    include!(concat!(env!("OUT_DIR"), "/phf_map_4.rs")),
    include!(concat!(env!("OUT_DIR"), "/phf_map_5.rs")),
    include!(concat!(env!("OUT_DIR"), "/phf_map_6.rs")),
    include!(concat!(env!("OUT_DIR"), "/phf_map_7.rs")),
    include!(concat!(env!("OUT_DIR"), "/phf_map_8.rs")),
    include!(concat!(env!("OUT_DIR"), "/phf_map_9.rs")),
    include!(concat!(env!("OUT_DIR"), "/phf_map_10.rs")),
];
