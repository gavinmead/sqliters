#[derive(Clone, Debug, PartialEq)]
pub enum CacheType {
    Shared,
    Private,
}

impl From<&str> for CacheType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "shared" => CacheType::Shared,
            "private" => CacheType::Private,
            _ => panic!("invalid cache_type: {}", value),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    ReadOnly,
    ReadWrite,
    ReadWriteCreate,
    Memory,
}

impl From<&str> for Mode {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "ro" => Mode::ReadOnly,
            "rw" => Mode::ReadWrite,
            "rwc" => Mode::ReadWriteCreate,
            "memory" => Mode::Memory,
            _ => panic!("invalid mode: {}", value),
        }
    }
}

// pub struct ConnectionOptions {
//     cache_type: CacheType,
//     immutable: bool,
//     mode: Mode,
//     mode_of: String,
//     no_lock: bool,
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_from_str() {
        let cases: Vec<(&str, Mode)> = vec![
            ("memory", Mode::Memory),
            ("Memory", Mode::Memory),
            ("mEmOry", Mode::Memory),
            ("MEMORY", Mode::Memory),
            ("ro", Mode::ReadOnly),
            ("rO", Mode::ReadOnly),
            ("RO", Mode::ReadOnly),
            ("rw", Mode::ReadWrite),
            ("RW", Mode::ReadWrite),
            ("rW", Mode::ReadWrite),
            ("Rw", Mode::ReadWrite),
            ("rwc", Mode::ReadWriteCreate),
            ("RWC", Mode::ReadWriteCreate),
            ("rWc", Mode::ReadWriteCreate),
        ];
        for case in cases {
            let mode = Mode::from(case.0);
            assert_eq!(mode, case.1)
        }
    }

    #[test]
    #[should_panic(expected = "invalid mode: will_fail")]
    fn mode_from_str_fail() {
        let _mode = Mode::from("will_fail");
    }

    #[test]
    fn cache_type_from_str() {
        let cases: Vec<(&str, CacheType)> = vec![
            ("shared", CacheType::Shared),
            ("SHARED", CacheType::Shared),
            ("sHaReD", CacheType::Shared),
            ("private", CacheType::Private),
            ("PRIVATE", CacheType::Private),
            ("pRiVaTe", CacheType::Private),
        ];
        for case in cases {
            let ct: CacheType = CacheType::from(case.0);
            assert_eq!(ct, case.1);
        }
    }

    #[test]
    #[should_panic(expected = "invalid cache_type: will_fail")]
    fn cache_type_fail() {
        let _ct = CacheType::from("will_fail");
    }
}
