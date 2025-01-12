use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref ON_DEMAND_FILE_REGEX: Regex =
        Regex::new(r#"['|"]{1}ondemand\.s['|"]{1}:\s*['|"]{1}([\w]*)['|"]{1}"#).unwrap();
    pub static ref INDICES_REGEX: Regex = Regex::new(r#"(\(\w{1}\[(\d{1,2})\],\s*16\))+"#).unwrap();
}

// pub const ADDITIONAL_RANDOM_NUMBER: i64 = 3;
// pub const DEFAULT_KEYWORD:&str = "obfiowerehiring";

pub struct ClientTransaction {
    pub ADDITIONAL_RANDOM_NUMBER: i64,
    pub DEFAULT_KEYWORD: String,
    pub DEFAULT_ROW_INDEX: usize,
    pub DEFAULT_KEY_BYTES_INDICES: Vec<usize>,
}

impl ClientTransaction {
    pub fn new() -> ClientTransaction {
        todo!()
    }
}
