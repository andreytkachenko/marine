use std::sync::{ Arc };
use super::{ PersonService, NewPerson };
use nickel::{ Request, Response };
use nickel::status::StatusCode;
use std::io::Read;
use serde_json;

pub struct PersonController {
    person_service: Arc<PersonService>
}

impl PersonController {
    pub fn new(person_service: Arc<PersonService>) -> Self {
        PersonController { person_service }
    }
    
    pub fn get_persons(&self, _req: &Request, _res: &Response) -> Result<String, StatusCode> {  	
        let person = try!(self.person_service.get_persons().map_err(|_| StatusCode::InternalServerError));
        
        Ok(try!(serde_json::to_string(&person).map_err(|_| StatusCode::InternalServerError)))
    }
    
    pub fn get_person(&self, req: &Request, _res: &Response) -> Result<String, StatusCode> {   	
        let id_s = req.param("id").unwrap();
        let id = id_s.parse::<i32>().unwrap();        
        let person = try!(self.person_service.get_person_by_id(id).map_err(|_| StatusCode::NotFound));
        
        Ok(try!(serde_json::to_string(&person).map_err(|_| StatusCode::InternalServerError)))
    }
    
    pub fn create_person(&self, req: &mut Request, _res: &Response) -> Result<String, StatusCode> {
        let mut form_data = String::new();
        let _x = req.origin.read_to_string(&mut form_data);
        
        let new_person = try!(serde_json::from_str::<NewPerson>(&form_data).map_err(|_| StatusCode::BadRequest));
        let person_id = try!(self.person_service.create_person(&new_person).map_err(|_| StatusCode::InternalServerError));
        let person = try!(self.person_service.get_person_by_id(person_id).map_err(|_| StatusCode::InternalServerError));
        
        Ok(try!(serde_json::to_string(&person).map_err(|_| StatusCode::InternalServerError)))
    }
    
    pub fn delete_person(&self, req: &mut Request, _res: &Response) -> Result<String, StatusCode> {
        let id_s = req.param("id").unwrap();
        let id = id_s.parse::<i32>().unwrap();
        let count = try!(self.person_service.delete_person(id).map_err(|_| StatusCode::InternalServerError));
        
        Ok(count.to_string())
    }
}