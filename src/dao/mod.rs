
// DAO traits/interfaces
mod daos;

pub use self::daos::ChannelDao;
pub use self::daos::DataDao;
pub use self::daos::FixtureDao;
pub use self::daos::LayoutDao;
pub use self::daos::PermissionDao;
pub use self::daos::ProjectDao;
pub use self::daos::SectionDao;
pub use self::daos::SequenceDao;
pub use self::daos::UserDao;

// Postgres implementations
mod daos_postgres;
mod channel_dao_postgres;
mod data_dao_postgres;
mod fixture_dao_postgres;
mod layout_dao_postgres;
mod permission_dao_postgres;
mod project_dao_postgres;
mod section_dao_postgres;
mod sequence_dao_postgres;
mod user_dao_postgres;

pub use self::daos_postgres::ChannelDaoPostgres;
pub use self::daos_postgres::DataDaoPostgres;
pub use self::daos_postgres::FixtureDaoPostgres;
pub use self::daos_postgres::LayoutDaoPostgres;
pub use self::daos_postgres::PermissionDaoPostgres;
pub use self::daos_postgres::ProjectDaoPostgres;
pub use self::daos_postgres::SectionDaoPostgres;
pub use self::daos_postgres::SequenceDaoPostgres;
pub use self::daos_postgres::UserDaoPostgres;
