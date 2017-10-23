#![feature(get_type_id, box_syntax)]
#![allow(non_upper_case_globals)]

extern crate serde;
extern crate dotenv;
extern crate chrono;
extern crate serde_json;

#[macro_use] extern crate nickel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate lazy_static;

mod locator;
mod service;
mod person;
mod connection;
mod schema;
mod routes;

use nickel::Nickel;
use service::service_locator;
use locator::Locator;
use routes::build_routes;

lazy_static! {
    static ref loc: Locator = service_locator();
}

fn main () {
    let mut server = Nickel::new();

	build_routes(&loc, &mut server);

    server.listen("127.0.0.1:6767").unwrap();
}
