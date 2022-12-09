use rocket::{Route};

mod identity;

pub fn routes() -> Vec<Route> {
    return routes![
      identity::get_identity
    ];
}