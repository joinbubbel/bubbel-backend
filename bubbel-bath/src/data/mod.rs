use super::*;

mod user;

pub use user::User;

pub struct DataState {
    pub db: PgConnection,
}

impl DataState {
    pub fn new(db: PgConnection) -> Self {
        DataState { db }
    }
}

//  Diesel doesn't allow implementing PartialEq or Eq for error types, so let's do it ourselves.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseError {
    Database(DatabaseErrorKind),
    NotFound,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseErrorKind {
    UniqueViolation,
    ForeignKeyViolation,
    UnableToSendCommand,
    SerializationFailure,
    ReadOnlyTransaction,
    NotNullViolation,
    CheckViolation,
    ClosedConnection,
    Unknown(String),
}

impl From<diesel::result::Error> for DatabaseError {
    fn from(value: diesel::result::Error) -> Self {
        match value {
            diesel::result::Error::DatabaseError(kind, _) => DatabaseError::Database(match kind {
                diesel::result::DatabaseErrorKind::UniqueViolation => {
                    DatabaseErrorKind::UniqueViolation
                }
                diesel::result::DatabaseErrorKind::ForeignKeyViolation => {
                    DatabaseErrorKind::ForeignKeyViolation
                }
                diesel::result::DatabaseErrorKind::UnableToSendCommand => {
                    DatabaseErrorKind::UnableToSendCommand
                }
                diesel::result::DatabaseErrorKind::SerializationFailure => {
                    DatabaseErrorKind::SerializationFailure
                }
                diesel::result::DatabaseErrorKind::ReadOnlyTransaction => {
                    DatabaseErrorKind::ReadOnlyTransaction
                }
                diesel::result::DatabaseErrorKind::NotNullViolation => {
                    DatabaseErrorKind::NotNullViolation
                }
                diesel::result::DatabaseErrorKind::CheckViolation => {
                    DatabaseErrorKind::CheckViolation
                }
                diesel::result::DatabaseErrorKind::ClosedConnection => {
                    DatabaseErrorKind::ClosedConnection
                }
                e => DatabaseErrorKind::Unknown(format!("{:#?}", e)),
            }),
            diesel::result::Error::NotFound => DatabaseError::NotFound,
            e => DatabaseError::Unknown(format!("{:#?}", e)),
        }
    }
}
