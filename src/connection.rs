use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::sync::Mutex;
use diesel::connection::{ Connection as DieselConnection };

pub type Connection = Mutex<SqliteConnection>;

pub fn default_connection() -> Mutex<SqliteConnection> {
	dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
	
    let conn = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to 'marine.db'"));
        
    Mutex::new(conn)
}