use postgres::{Connection, TlsMode};

use error::Error;

const postgres_conn_str: &'static str = "postgresql://proton:1234qwermnbv@localhost/proton_cli";


pub fn get_connection() -> Result<Connection, Error> {
    Connection::connect(postgres_conn_str, TlsMode::None)
        .map_err(Error::Postgres)
}
