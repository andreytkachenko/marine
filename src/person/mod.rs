mod controller;
mod service;

use super::schema::person;
pub use self::service::PersonService;
pub use self::controller::PersonController;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub create_time: NaiveDateTime
}

#[derive(Insertable)]
#[derive(Serialize, Deserialize)]
#[table_name = "person"]
pub struct NewPerson {
    pub name: String,
    pub create_time: NaiveDateTime
}