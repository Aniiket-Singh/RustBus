use diesel::prelude::*;
use crate::config::Config;

pub struct Store {
    pub conn: PgConnection
}
//sort of a constructor function
// impl Default for Store{
impl Store{
    // Store::default is accessed in api/main.rs to get db conn
    pub fn default() -> Result<Self, ConnectionError>{
        let config = Config::default();

        let connection = PgConnection::establish(&config.db_url)?;
        Ok(Self {
            conn: connection
        })
    }  
}