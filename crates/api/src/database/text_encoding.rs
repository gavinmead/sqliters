#[derive(Debug, PartialEq)]
pub enum TextEncoding {
    UTF8,
    UTF16BE,
    UTF16LE,
}

impl From<u32> for TextEncoding {
    fn from(value: u32) -> Self {
        match value {
            1 => TextEncoding::UTF8,
            2 => TextEncoding::UTF16LE,
            3 => TextEncoding::UTF16BE,
            _ => panic!("unsupported text encoding: {}", value),
        }
    }
}

impl From<TextEncoding> for u32 {
    fn from(value: TextEncoding) -> Self {
        match value {
            TextEncoding::UTF8 => 1,
            TextEncoding::UTF16LE => 2,
            TextEncoding::UTF16BE => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u32_ok() {
        let cases = vec![
            (1, TextEncoding::UTF8),
            (2, TextEncoding::UTF16LE),
            (3, TextEncoding::UTF16BE),
        ];
        for case in cases {
            let result: TextEncoding = case.0.into();
            assert_eq!(result, case.1);
        }
    }

    #[test]
    #[should_panic(expected = "unsupported text encoding: 42")]
    fn from_u32_fail() {
        let _result: TextEncoding = 42.into();
    }

    #[test]
    fn to_u32_ok() {
        let cases: Vec<(TextEncoding, u32)> = vec![
            (TextEncoding::UTF8, 1),
            (TextEncoding::UTF16LE, 2),
            (TextEncoding::UTF16BE, 3),
        ];
        for case in cases {
            let result: u32 = case.0.into();
            assert_eq!(result, case.1);
        }
    }
}
