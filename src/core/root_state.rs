use diesel::r2d2::{ConnectionManager, Pool};
use diesel::MysqlConnection;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

#[derive(Clone)]
pub struct RootState {
    pub db_pool: DbPool,
}

