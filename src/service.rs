use locator::Locator;
use person::{ PersonService, PersonController };
use connection::default_connection;

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Service {
    CONNECTION,
    PERSON_CONTROLLER,
    PERSON_SERVICE,
}

impl Eq for Service {}

pub fn service_locator() -> Locator {
    let mut loc = Locator::new();
    
    let _connection = loc.set(Service::CONNECTION, default_connection());
    let _person_service = loc.set(Service::PERSON_SERVICE, PersonService::new(_connection));
    let _person_controller = loc.set(Service::PERSON_CONTROLLER, PersonController::new(_person_service));
    
    loc
}