use std::env;
use std::error::Error as StdError;
use diesel::pg::Pg;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

/// A collection of operations on the database

use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing::{debug, info};

mod schema;
pub mod user;
pub mod games;


pub fn create_connection_pool() -> r2d2::Pool<ConnectionManager<PgConnection>>{
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}


#[derive(Debug)]
pub enum DatabaseError {
    UniqueKeyViolation,
    NotFound,
    Other(Box<dyn StdError>),
}

impl From<DieselError> for DatabaseError {
    fn from(value: DieselError) -> Self {
        match value {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                Self::UniqueKeyViolation
            }
            DieselError::NotFound => Self::NotFound,
            err => Self::Other(Box::new(err)),
        }
    }
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");

pub fn run_migrations(conn: &mut impl MigrationHarness<Pg>) -> Result<(), Box<dyn StdError + Send + Sync + 'static>> {
    let pending_migrations = conn.run_pending_migrations(MIGRATIONS)?;
    if pending_migrations.len() > 0 {
        info!("Sucessfully ran {} database migrations", pending_migrations.len());
    } else {
        debug!("No pending migrations to run")
    }

    Ok(())
}