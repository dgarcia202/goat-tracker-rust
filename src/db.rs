use std::env;
use std::ops::Deref;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use dotenv::dotenv;

pub struct DbConn(pub PooledConnection<ConnectionManager<SqliteConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<SqlitePool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }    
}

impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool() -> SqlitePool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::new(manager).expect("Error creating DB connection pool")
}