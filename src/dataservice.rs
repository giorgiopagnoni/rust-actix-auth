use crate::controllers::RegisterUserRequest;

use r2d2_mysql::MysqlConnectionManager;
use mysql::{OptsBuilder, params};

use actix_web::web::Json;
use uuid::Uuid;
use bcrypt::hash;

pub type DS = MysqlR2D2DataService;

pub trait DataService {
    fn is_email_taken(&self, email: &str) -> bool;
    fn persist_new_user(&self, usr_req: &Json<RegisterUserRequest>) -> String;
    fn activate_by_token(&self, token: &str) -> bool;
}

#[derive(Clone, Debug)]
pub struct MysqlR2D2DataService {
    db_pool: r2d2::Pool<MysqlConnectionManager>
}

impl MysqlR2D2DataService {
    pub fn new() -> MysqlR2D2DataService {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let opt = mysql::Opts::from_url(&db_url).unwrap();
        let manager = r2d2_mysql::MysqlConnectionManager::new(OptsBuilder::from_opts(opt));
        let db_pool = r2d2::Pool::builder().build(manager).expect("Failed to create DB connection pool");

        MysqlR2D2DataService {
            db_pool
        }
    }
}

impl DataService for MysqlR2D2DataService {
    fn is_email_taken(&self, email: &str) -> bool {
        let mut conn = self.db_pool.get().unwrap();

        if conn.prep_exec("SELECT * FROM auth_users WHERE email = :email LIMIT 1",
                          params! {"email" => &email})
            .unwrap().next().is_some() {
            return true;
        }

        return false;
    }

    fn persist_new_user(&self, usr_req: &Json<RegisterUserRequest>) -> String {
        let token = Uuid::new_v4().to_string();
        let passwd = hash(&usr_req.password, 6).unwrap();

        let mut conn = self.db_pool.get().unwrap();
        conn.prep_exec(r"INSERT INTO auth_users (email, passwd, token)
                            VALUES (:email, :passwd, :token)",
                       params! {
                "email" => &usr_req.email,
                "passwd" => &passwd,
                "token" => &token
            }).unwrap();


        token
    }

    fn activate_by_token(&self, token: &str) -> bool {
        let mut conn = self.db_pool.get().unwrap();

        let r = conn.prep_exec(r"
            UPDATE auth_users
            SET is_active = 1, token = NULL
            WHERE token = :token", params! {
                "token" => &token,
            }).unwrap().affected_rows();

        return r > 0
    }
}
