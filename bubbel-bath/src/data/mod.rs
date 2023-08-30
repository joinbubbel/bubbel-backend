use super::*;
use std::string::ToString;

mod club_members;
mod club_profile;
mod friend_connection;
mod message_room;
mod message_room_members;
mod user;
mod user_profile;

pub use club_members::*;
pub use club_profile::*;
pub use friend_connection::*;
pub use message_room::*;
pub use message_room_members::*;
pub use user::*;
pub use user_profile::*;

/// Holds the state with the database connection pool.
/// Use [`DataStateInstance`] for one off connections.
pub struct DataState {
    pool: Pool<ConnectionManager<PgConnection>>,
    user_salt: String,
}

impl DataState {
    pub fn new(db_url: &str, user_salt: &str) -> Self {
        let manager = ConnectionManager::new(db_url);
        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Failed to build database connection pool.");
        DataState {
            pool,
            user_salt: user_salt.to_owned(),
        }
    }

    pub fn spawn(&self) -> DataStateInstance {
        let db = self.pool.get().unwrap();
        DataStateInstance {
            db,
            user_salt: self.user_salt.clone(),
        }
    }
}

/// Holds the state required to connect to the database.
pub struct DataStateInstance {
    pub db: PooledConnection<ConnectionManager<PgConnection>>,
    pub user_salt: String,
}

/// All of these errors are directly from [`diesel`](https://docs.rs/diesel/latest/diesel/result/enum.Error.html).
/// Errors that should not be possible are handled with [`DatabaseError::Internal`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseError {
    NotFound,
    UniqueViolation,
    ForeignKeyViolation,
    NotNullViolation,
    CheckViolation,
    Internal { ierror: String },
}

impl ToString for DatabaseError {
    fn to_string(&self) -> String {
        match self {
            DatabaseError::Internal { ierror } => ierror.clone(),
            DatabaseError::NotFound => "NotFound".to_owned(),
            DatabaseError::UniqueViolation => "UniqueViolation".to_owned(),
            DatabaseError::ForeignKeyViolation => "ForeignKeyViolation".to_owned(),
            DatabaseError::NotNullViolation => "NotNullViolation".to_owned(),
            DatabaseError::CheckViolation => "CheckViolation".to_owned(),
        }
    }
}

impl From<diesel::result::Error> for DatabaseError {
    fn from(value: diesel::result::Error) -> Self {
        match value {
            diesel::result::Error::NotFound => Self::NotFound,
            diesel::result::Error::DatabaseError(kind, _) => match kind {
                diesel::result::DatabaseErrorKind::UniqueViolation => Self::UniqueViolation,
                diesel::result::DatabaseErrorKind::ForeignKeyViolation => Self::ForeignKeyViolation,
                diesel::result::DatabaseErrorKind::NotNullViolation => Self::NotNullViolation,
                diesel::result::DatabaseErrorKind::CheckViolation => Self::CheckViolation,
                _ => Self::Internal {
                    ierror: value.to_string(),
                },
            },
            _ => Self::Internal {
                ierror: value.to_string(),
            },
        }
    }
}
