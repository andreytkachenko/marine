use super::{ NewPerson, Person };
use std::sync::Arc;
use connection::Connection;
use schema::person::dsl::*;
use schema::person;
use diesel::prelude::*;
use diesel;
use diesel::types;
use diesel::result;

pub struct PersonService {
    connection: Arc<Connection>
}

impl PersonService {
	pub fn new (connection: Arc<Connection>) -> Self {
		PersonService {
			connection
		}
	}
	
	pub fn get_persons(&self) -> Result<Vec<Person>, result::Error> {
        let conn = self.connection.lock().unwrap();
        
	    person
	        .get_results::<Person>(&*conn)
    }
	
    pub fn get_person_by_id(&self, req_id: i32) -> Result<Person, result::Error> {
        let conn = self.connection.lock().unwrap();
        
	    person
	        .find(req_id)
	        .get_result::<Person>(&*conn)
    }
    
    pub fn create_person(&self, new_person: &NewPerson) -> Result<i32, result::Error> {
    	let conn = self.connection.lock().unwrap();

	    diesel::insert(new_person)
		    .into(person::table)
	        .execute(&*conn)
	        .expect("Error saving new person");
	        
        no_arg_sql_function!(LAST_INSERT_ROWID, types::Integer);
	        
        diesel::select(LAST_INSERT_ROWID).first(&*conn)
    }
    
    pub fn delete_person(&self, del_id: i32) -> Result<usize, result::Error> {
    	let conn = self.connection.lock().unwrap();
    	
    	diesel::delete(person.filter(id.eq(del_id))).execute(&*conn)
    }
}