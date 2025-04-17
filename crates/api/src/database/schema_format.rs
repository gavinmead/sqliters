#[derive(Debug, PartialEq)]
pub enum SchemaFormat {
    V1,
    V2,
    V3,
    V4,
}

impl From<u32> for SchemaFormat {
    fn from(v: u32) -> Self {
        match v {
            1 => SchemaFormat::V1,
            2 => SchemaFormat::V2,
            3 => SchemaFormat::V3,
            4 => SchemaFormat::V4,
            _ => panic!("unsupported schema format: {}", v),
        }
    }
}

impl From<SchemaFormat> for u32 {
    fn from(value: SchemaFormat) -> Self {
        match value {
            SchemaFormat::V1 => 1,
            SchemaFormat::V2 => 2,
            SchemaFormat::V3 => 3,
            SchemaFormat::V4 => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_format_to_u32() {
        let cases = vec![
            (SchemaFormat::V1, 1),
            (SchemaFormat::V2, 2),
            (SchemaFormat::V3, 3),
            (SchemaFormat::V4, 4),
        ];
        for case in cases {
            let result: u32 = case.0.into();
            assert_eq!(result, case.1)
        }
    }

    #[test]
    fn schema_format_from_u32() {
        let cases = vec![
            (1, SchemaFormat::V1),
            (2, SchemaFormat::V2),
            (3, SchemaFormat::V3),
            (4, SchemaFormat::V4),
        ];
        for case in cases {
            let result: SchemaFormat = case.0.into();
            assert_eq!(result, case.1)
        }
    }

    #[test]
    #[should_panic(expected = "unsupported schema format: 42")]
    fn schema_format_from_u32_err() {
        let _result: SchemaFormat = 42.into();
    }
}
