use super::*;

mod user;

pub use user::User;

pub struct DataState {
    pub db: PgConnection,
    pub user_salt: String,
}

impl DataState {
    pub fn new(db_url: &str, user_salt: &str) -> Result<Self, ConnectionError> {
        let db = PgConnection::establish(db_url)?;
        Ok(DataState {
            db,
            user_salt: user_salt.to_owned(),
        })
    }
}

//  Diesel doesn't allow implementing PartialEq or Eq for error types, so let's do it ourselves.

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum DatabaseError {
    NotFound,
    Unknown { uerror: String },

    UniqueViolation,
    ForeignKeyViolation,
    UnableToSendCommand,
    SerializationFailure,
    ReadOnlyTransaction,
    NotNullViolation,
    CheckViolation,
    ClosedConnection,
}

impl From<diesel::result::Error> for DatabaseError {
    fn from(value: diesel::result::Error) -> Self {
        match value {
            diesel::result::Error::DatabaseError(kind, _) => match kind {
                diesel::result::DatabaseErrorKind::UniqueViolation => {
                    DatabaseError::UniqueViolation
                }
                diesel::result::DatabaseErrorKind::ForeignKeyViolation => {
                    DatabaseError::ForeignKeyViolation
                }
                diesel::result::DatabaseErrorKind::UnableToSendCommand => {
                    DatabaseError::UnableToSendCommand
                }
                diesel::result::DatabaseErrorKind::SerializationFailure => {
                    DatabaseError::SerializationFailure
                }
                diesel::result::DatabaseErrorKind::ReadOnlyTransaction => {
                    DatabaseError::ReadOnlyTransaction
                }
                diesel::result::DatabaseErrorKind::NotNullViolation => {
                    DatabaseError::NotNullViolation
                }
                diesel::result::DatabaseErrorKind::CheckViolation => DatabaseError::CheckViolation,
                diesel::result::DatabaseErrorKind::ClosedConnection => {
                    DatabaseError::ClosedConnection
                }
                e => DatabaseError::Unknown {
                    uerror: format!("(UKi) {:#?}", e),
                },
            },
            diesel::result::Error::NotFound => DatabaseError::NotFound,
            e => DatabaseError::Unknown {
                uerror: format!("(U) {:#?}", e),
            },
        }
    }
}
