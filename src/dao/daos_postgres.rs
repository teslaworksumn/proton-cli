use postgres::{Connection, TlsMode};

use error::Error;
use project_types::Channel;


const postgres_conn_str: &'static str = "postgresql://proton:1234qwermnbv@localhost/proton_cli";

pub struct DaoPostgres {
    conn: Connection
}

pub type ChannelDaoPostgres = DaoPostgres;
pub type FixtureDaoPostgres = DaoPostgres;
pub type LayoutDaoPostgres = DaoPostgres;
pub type PermissionDaoPostgres = DaoPostgres;
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
    Connection::connect(postgres_conn_str, TlsMode::None)
        .map_err(Error::Postgres)
}

