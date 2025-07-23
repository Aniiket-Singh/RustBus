use std::env;
use diesel::prelude::*;

pub mod schema;

pub struct Store {
    pub conn: PgConnection
}

//sort of a constructor function
// impl Default for Store{
impl Store{
    fn default() -> Result<Self, ConnectionError> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL variable not loaded."));
        let connection = PgConnection::establish(&db_url)?;
        Ok(Self {
            conn: connection
        })
    }

    pub fn create_user(&self){
        print!("create user logic")
    }

    pub fn create_website(&self) -> String{
        String::from("1")
    }
}