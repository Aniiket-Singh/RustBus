// only checks if the program can access dATABASE_URL from .env, if it does db_url is accessed using config module
use std::env;

pub struct Config {
    pub db_url: String
}

impl Default for Config{
    fn default() -> Self {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL variable not loaded."));
        Self{
            db_url
        }
    }
}