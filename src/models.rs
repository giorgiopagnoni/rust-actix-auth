use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Debug, Queryable)]
pub struct User {
    id: u32,
    email: String,
    passwd: String,
    token: Option<String>
}