use crate::database::file_format::{FileFormatReadVersion, FileFormatWriteVersion};
use crate::database::page_size::PageSize;
use crate::errors::SqliteResult;
use crate::SqliteError;
use bytes::{BufMut, Bytes, BytesMut};

#[allow(dead_code)]
type HeaderFieldWriter = fn(header: &SqliteHeader, bytes: &mut BytesMut) -> Result<(), SqliteError>;

fn write_header_string(header: &SqliteHeader, bytes: &mut BytesMut) -> Result<(), SqliteError> {
    bytes.put(header.header.as_bytes());
    Ok(())
}

fn write_page_size(header: &SqliteHeader, bytes: &mut BytesMut) -> Result<(), SqliteError> {
    bytes.put_u16(header.page_size.into());
    Ok(())
}

fn write_file_format_write_version(
    header: &SqliteHeader,
    bytes: &mut BytesMut,
) -> Result<(), SqliteError> {
    bytes.put_u8(header.file_format_write_version.into());
    Ok(())
}

fn write_file_format_read_version(
    header: &SqliteHeader,
    bytes: &mut BytesMut,
) -> Result<(), SqliteError> {
    bytes.put_u8(header.file_format_read_version.into());
    Ok(())
}

const WRITERS: [HeaderFieldWriter; 4] = [
    write_header_string,
    write_page_size,
    write_file_format_write_version,
    write_file_format_read_version,
];

/// Represents the header section of the database per https://sqlite.org/fileformat2.html
#[derive(Clone, Debug, PartialEq)]
pub struct SqliteHeader {
    header: String,
    page_size: PageSize,
    file_format_write_version: FileFormatWriteVersion,
    file_format_read_version: FileFormatReadVersion,
}

impl SqliteHeader {
    /// Given a Byte buffer, create a SqliteHeader struct
    #[allow(dead_code)]
    fn from_buffer(_buf: &Bytes) -> SqliteResult<SqliteHeader> {
        todo!()
    }

    /// Given a mutable Byte buffer, write the contents of the header starting at position 0
    /// in the buffer
    #[allow(dead_code)]
    fn write(&self, buf: &mut BytesMut) -> SqliteResult<()> {
        for writer in WRITERS {
            writer(self, buf)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Buf;
    use std::io::Cursor;
    fn test_header() -> SqliteHeader {
        SqliteHeader {
            header: String::from("SQLite format 3\0"),
            page_size: PageSize::Size512,
            file_format_write_version: FileFormatWriteVersion::Legacy,
            file_format_read_version: FileFormatReadVersion::Legacy,
        }
    }

    #[test]
    fn write_header_string_ok() {
        let header = test_header();
        let mut buf: BytesMut = BytesMut::with_capacity(100);
        let result = write_header_string(&header, &mut buf);
        assert!(result.is_ok());
        assert_eq!(buf.len(), 16);
        assert_eq!(buf[0..16].as_ref(), header.header.as_bytes());
    }

    #[test]
    fn write_page_size_ok() {
        let header = test_header();
        let mut buf: BytesMut = BytesMut::with_capacity(100);

        let result = write_page_size(&header, &mut buf);
        assert!(result.is_ok());
        assert_eq!(buf.len(), 2);
        let mut cursor = Cursor::new(&buf[..]);
        cursor.set_position(0);
        let actual = cursor.get_u16();
        assert_eq!(actual, header.page_size.into());
    }

    #[test]
    fn write_file_format_write_version_ok() {
        let header = test_header();
        let mut buf: BytesMut = BytesMut::with_capacity(100);
        let result = write_file_format_write_version(&header, &mut buf);
        assert!(result.is_ok());
        assert_eq!(buf.len(), 1);
        let mut cursor = Cursor::new(&buf[..]);
        cursor.set_position(0);
        let actual = cursor.get_u8();
        assert_eq!(actual, header.file_format_write_version.into());
    }

    #[test]
    fn write_file_format_read_version_ok() {
        let header = test_header();
        let mut buf: BytesMut = BytesMut::with_capacity(100);
        let result = write_file_format_read_version(&header, &mut buf);
        assert!(result.is_ok());
        assert_eq!(buf.len(), 1);
        let mut cursor = Cursor::new(&buf[..]);
        cursor.set_position(0);
        let actual = cursor.get_u8();
        assert_eq!(actual, header.file_format_read_version.into());
    }

    //Integration tests
    #[test]
    fn write_ok() {
        let header = test_header();
        let mut buf: BytesMut = BytesMut::with_capacity(100);
        let result = header.write(&mut buf);
        let expected_buf_capacity = 20;
        assert!(result.is_ok());
        assert_eq!(buf.len(), expected_buf_capacity);
    }
}
