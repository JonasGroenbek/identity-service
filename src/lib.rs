use futures::Future;
use rocket::{Rocket, Ignite, Error};

#[macro_use] extern crate rocket;

pub mod routes;
pub mod mongo;

pub fn launch() -> impl Future<Output = Result<Rocket<Ignite>, Error>> {
    rocket::build().mount("/", routes::routes()).launch()
}