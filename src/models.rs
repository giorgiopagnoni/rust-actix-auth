use r2d2_mysql::MysqlConnectionManager;

pub type Pool = r2d2::Pool<MysqlConnectionManager>;

#[derive(Debug)]
pub struct User {
    id: u32,
    email: String,
    passwd: String,
    token: Option<String>
}