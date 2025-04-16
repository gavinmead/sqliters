pub type SqliteResult<T, E = SqliteError> = Result<T, E>;

///Sqlite specific errors
#[derive(Debug)]
pub enum SqliteError {
    Error { code: i32, message: String },
    CannotOpen { code: i32, message: String },
}
