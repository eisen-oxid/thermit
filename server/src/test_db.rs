// See https://docs.diesel.rs/1.4.x/src/diesel/r2d2.rs.html
// We added the distinction between test and not test config to be able to use test_connections

use diesel::r2d2::{Error, ManageConnection};
use diesel::{Connection, PgConnection};
use diesel_migrations::run_pending_migrations;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CustomConnectionManager<T> {
    database_url: String,
    _marker: PhantomData<T>,
}

impl<T> CustomConnectionManager<T> {
    /// Returns a new connection manager,
    /// which establishes connections to the given database URL.
    pub fn new<S: Into<String>>(database_url: S) -> Self {
        CustomConnectionManager {
            database_url: database_url.into(),
            _marker: PhantomData,
        }
    }
}

unsafe impl<T: Send + 'static> Sync for CustomConnectionManager<T> {}

impl ManageConnection for CustomConnectionManager<PgConnection> {
    type Connection = PgConnection;
    type Error = diesel::r2d2::Error;

    fn connect(&self) -> Result<PgConnection, Error> {
        let conn = PgConnection::establish(&self.database_url)
            .map_err(Error::ConnectionError)
            .unwrap();
        run_pending_migrations(&conn).unwrap();
        conn.begin_test_transaction().unwrap();
        Ok(conn)
    }

    fn is_valid(&self, conn: &mut PgConnection) -> Result<(), Error> {
        conn.execute("SELECT 1")
            .map(|_| ())
            .map_err(Error::QueryError)
    }

    fn has_broken(&self, _conn: &mut PgConnection) -> bool {
        false
    }
}
