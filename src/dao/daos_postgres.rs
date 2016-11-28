use postgres::{Connection, TlsMode};

use error::Error;


const POSTGRES_CONN_STR: &'static str = "postgresql://proton:1234qwermnbv@localhost/proton_cli";

pub struct DaoPostgres {
    pub conn: Connection
}

pub type ChannelDaoPostgres = DaoPostgres;
pub type DataDaoPostgres = DaoPostgres;
pub type FixtureDaoPostgres = DaoPostgres;
pub type LayoutDaoPostgres = DaoPostgres;
pub type PermissionDaoPostgres = DaoPostgres;
pub type ProjectDaoPostgres = DaoPostgres;
pub type SectionDaoPostgres = DaoPostgres;
pub type SequenceDaoPostgres = DaoPostgres;
pub type UserDaoPostgres = DaoPostgres;


impl DaoPostgres {
    pub fn new() -> Result<DaoPostgres, Error> {
        let conn = try!(get_connection());
        Ok(DaoPostgres {
            conn: conn
        })
    }
}

/// Gets a new connection to the postgresql database
fn get_connection() -> Result<Connection, Error> {
    Connection::connect(POSTGRES_CONN_STR, TlsMode::None)
        .map_err(Error::PostgresConnection)
}

