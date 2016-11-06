
// DAO traits/interfaces
mod daos;

pub use self::daos::ChannelDao;
pub use self::daos::FixtureDao;
pub use self::daos::LayoutDao;
pub use self::daos::PermissionDao;
pub use self::daos::SectionDao;
pub use self::daos::SequenceDao;
pub use self::daos::UserDao;

// Postgres implementations
mod channel_dao_postgres;
mod fixture_dao_postgres;
mod layout_dao_postgres;
mod permission_dao_postgres;
mod section_dao_postgres;
mod sequence_dao_postgres;
mod user_dao_postgres;

pub use self::channel_dao_postgres::ChannelDaoPostgres;
pub use self::fixture_dao_postgres::FixtureDaoPostgres;
pub use self::layout_dao_postgres::LayoutDaoPostgres;
pub use self::permission_dao_postgres::PermissionDaoPostgres;
pub use self::section_dao_postgres::SectionDaoPostgres;
pub use self::sequence_dao_postgres::SequenceDaoPostgres;
pub use self::user_dao_postgres::UserDaoPostgres;

