#[derive(Clone, Debug, Copy, PartialEq)]
pub enum PageSize {
    Size512,
    Size1024,
    Size2048,
    Size4096,
    Size8192,
    Size16384,
    Size32768,
    Size65536,
}

impl From<PageSize> for u16 {
    fn from(value: PageSize) -> Self {
        match value {
            PageSize::Size512 => 512,
            PageSize::Size1024 => 1024,
            PageSize::Size2048 => 2048,
            PageSize::Size4096 => 4096,
            PageSize::Size8192 => 8192,
            PageSize::Size16384 => 16384,
            PageSize::Size32768 => 32768,
            PageSize::Size65536 => 1,
        }
    }
}

impl From<PageSize> for u32 {
    fn from(value: PageSize) -> Self {
        match value {
            PageSize::Size512 => 512,
            PageSize::Size1024 => 1024,
            PageSize::Size2048 => 2048,
            PageSize::Size4096 => 4096,
            PageSize::Size8192 => 8192,
            PageSize::Size16384 => 16384,
            PageSize::Size32768 => 32768,
            PageSize::Size65536 => 65536,
        }
    }
}

impl From<u32> for PageSize {
    fn from(value: u32) -> Self {
        match value {
            512 => PageSize::Size512,
            1024 => PageSize::Size1024,
            2048 => PageSize::Size2048,
            4096 => PageSize::Size4096,
            8192 => PageSize::Size8192,
            16384 => PageSize::Size16384,
            32768 => PageSize::Size32768,
            65536 => PageSize::Size65536,
            _ => panic!("unsupported page size: {}", value),
        }
    }
}

impl From<u16> for PageSize {
    fn from(value: u16) -> Self {
        match value {
            512 => PageSize::Size512,
            1024 => PageSize::Size1024,
            2048 => PageSize::Size2048,
            4096 => PageSize::Size4096,
            8192 => PageSize::Size8192,
            16384 => PageSize::Size16384,
            32768 => PageSize::Size32768,
            1 => PageSize::Size65536,
            _ => panic!("unsupported page size: {}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_u32() {
        let cases = vec![
            (PageSize::Size512, 512),
            (PageSize::Size1024, 1024),
            (PageSize::Size2048, 2048),
            (PageSize::Size4096, 4096),
            (PageSize::Size8192, 8192),
            (PageSize::Size16384, 16384),
            (PageSize::Size32768, 32768),
            (PageSize::Size65536, 65536),
        ];
        for case in cases {
            let ps = case.0;
            let result: u32 = ps.into();
            assert_eq!(result, case.1);
        }
    }

    #[test]
    fn test_into_u16() {
        let cases = vec![
            (PageSize::Size512, 512),
            (PageSize::Size1024, 1024),
            (PageSize::Size2048, 2048),
            (PageSize::Size4096, 4096),
            (PageSize::Size8192, 8192),
            (PageSize::Size16384, 16384),
            (PageSize::Size32768, 32768),
            (PageSize::Size65536, 1),
        ];
        for case in cases {
            let ps = case.0;
            let result: u16 = ps.into();
            assert_eq!(result, case.1);
        }
    }

    #[test]
    fn test_from_u32() {
        let cases = vec![
            (PageSize::Size512, 512),
            (PageSize::Size1024, 1024),
            (PageSize::Size2048, 2048),
            (PageSize::Size4096, 4096),
            (PageSize::Size8192, 8192),
            (PageSize::Size16384, 16384),
            (PageSize::Size32768, 32768),
            (PageSize::Size65536, 65536),
        ];
        for case in cases {
            let page_size: u32 = case.1;
            let ps: PageSize = page_size.into();
            assert_eq!(ps, case.0);
        }
    }

    #[test]
    #[should_panic(expected = "unsupported page size: 42")]
    fn test_from_u32_unsupported() {
        let page_size: u32 = 42;
        let _ps: PageSize = page_size.into();
    }

    #[test]
    fn test_from_u16() {
        let cases: Vec<(PageSize, u16)> = vec![
            (PageSize::Size512, 512),
            (PageSize::Size1024, 1024),
            (PageSize::Size2048, 2048),
            (PageSize::Size4096, 4096),
            (PageSize::Size8192, 8192),
            (PageSize::Size16384, 16384),
            (PageSize::Size32768, 32768),
            (PageSize::Size65536, 1),
        ];
        for case in cases {
            let ps = case.1;
            let ps: PageSize = ps.into();
            assert_eq!(ps, case.0);
        }
    }

    #[test]
    #[should_panic(expected = "unsupported page size: 42")]
    fn test_from_u16_unsupported() {
        let page_size: u16 = 42;
        let _ps: PageSize = page_size.into();
    }
}
