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

fn write_page_reserved_space(
    header: &SqliteHeader,
    bytes: &mut BytesMut,
) -> Result<(), SqliteError> {
    bytes.put_u8(header.page_reserved_space);
    Ok(())
}

fn write_max_embedded_payload_fraction(
    header: &SqliteHeader,
    bytes: &mut BytesMut,
) -> Result<(), SqliteError> {
    bytes.put_u8(header.max_embedded_payload_fraction);
    Ok(())
}

fn write_min_embedded_payload_fraction(
    header: &SqliteHeader,
    bytes: &mut BytesMut,
) -> Result<(), SqliteError> {
    bytes.put_u8(header.min_embedded_payload_fraction);
    Ok(())
}

fn write_leaf_payload_fraction(
    header: &SqliteHeader,
    bytes: &mut BytesMut,
) -> Result<(), SqliteError> {
    bytes.put_u8(header.leaf_payload_fraction);
    Ok(())
}

fn write_file_change_counter(
    header: &SqliteHeader,
    bytes: &mut BytesMut,
) -> Result<(), SqliteError> {
    bytes.put_u32(header.file_change_counter);
    Ok(())
}

fn write_size_in_pages(header: &SqliteHeader, bytes: &mut BytesMut) -> Result<(), SqliteError> {
    bytes.put_u32(header.size_in_pages);
    Ok(())
}

const WRITERS: [HeaderFieldWriter; 10] = [
    write_header_string,
    write_page_size,
    write_file_format_write_version,
    write_file_format_read_version,
    write_page_reserved_space,
    write_max_embedded_payload_fraction,
    write_min_embedded_payload_fraction,
    write_leaf_payload_fraction,
    write_file_change_counter,
    write_size_in_pages,
];

/// Represents the header section of the database per https://sqlite.org/fileformat2.html
#[derive(Clone, Debug, PartialEq)]
pub struct SqliteHeader {
    header: String,
    page_size: PageSize,
    file_format_write_version: FileFormatWriteVersion,
    file_format_read_version: FileFormatReadVersion,
    page_reserved_space: u8,
    max_embedded_payload_fraction: u8,
    min_embedded_payload_fraction: u8,
    leaf_payload_fraction: u8,
    file_change_counter: u32,
    size_in_pages: u32,
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
            page_reserved_space: 0,
            max_embedded_payload_fraction: 64,
            min_embedded_payload_fraction: 32,
            leaf_payload_fraction: 32,
            file_change_counter: 0,
            size_in_pages: 1,
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
        let expected_buf_capacity = 32;
        assert!(result.is_ok());
        assert_eq!(buf.len(), expected_buf_capacity);
    }
}
