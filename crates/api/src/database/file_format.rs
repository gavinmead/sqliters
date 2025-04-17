#[derive(Debug, PartialEq)]
pub enum FileFormatWriteVersion {
    Legacy,
    Wal,
}

impl From<u8> for FileFormatWriteVersion {
    fn from(value: u8) -> Self {
        match value {
            1 => FileFormatWriteVersion::Legacy,
            2 => FileFormatWriteVersion::Wal,
            _ => panic!("unsupported file format write version: {}", value),
        }
    }
}

impl From<FileFormatWriteVersion> for u8 {
    fn from(version: FileFormatWriteVersion) -> Self {
        match version {
            FileFormatWriteVersion::Legacy => 1,
            FileFormatWriteVersion::Wal => 2,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FileFormatReadVersion {
    Legacy,
    Wal,
}

impl From<u8> for FileFormatReadVersion {
    fn from(value: u8) -> Self {
        match value {
            1 => FileFormatReadVersion::Legacy,
            2 => FileFormatReadVersion::Wal,
            _ => panic!("unsupported file format read version: {}", value),
        }
    }
}

impl From<FileFormatReadVersion> for u8 {
    fn from(version: FileFormatReadVersion) -> Self {
        match version {
            FileFormatReadVersion::Legacy => 1,
            FileFormatReadVersion::Wal => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_format_write_version_ok() {
        let cases = vec![
            (1, FileFormatWriteVersion::Legacy),
            (2, FileFormatWriteVersion::Wal),
        ];
        for case in cases {
            let result = FileFormatWriteVersion::from(case.0);
            assert_eq!(result, case.1);
        }
    }

    #[test]
    #[should_panic(expected = "unsupported file format write version: 42")]
    fn file_format_write_version_err() {
        let _result = FileFormatWriteVersion::from(42);
    }

    #[test]
    fn file_format_write_version_to_u8_ok() {
        let cases: Vec<(FileFormatWriteVersion, u8)> = vec![
            (FileFormatWriteVersion::Legacy, 1),
            (FileFormatWriteVersion::Wal, 2),
        ];
        for case in cases {
            let result: u8 = case.0.into();
            assert_eq!(result, case.1);
        }
    }

    #[test]
    fn file_format_read_version_ok() {
        let cases = vec![
            (1, FileFormatReadVersion::Legacy),
            (2, FileFormatReadVersion::Wal),
        ];
        for case in cases {
            let result = FileFormatReadVersion::from(case.0);
            assert_eq!(result, case.1);
        }
    }

    #[test]
    #[should_panic(expected = "unsupported file format read version: 42")]
    fn file_format_read_version_err() {
        let _result = FileFormatReadVersion::from(42);
    }

    #[test]
    fn file_format_read_version_to_u8_ok() {
        let cases: Vec<(FileFormatReadVersion, u8)> = vec![
            (FileFormatReadVersion::Legacy, 1),
            (FileFormatReadVersion::Wal, 2),
        ];
        for case in cases {
            let result: u8 = case.0.into();
            assert_eq!(result, case.1);
        }
    }
}
