use identity_service;
use crate::identity_service::mongo::database::start_database;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    start_database();

    let _future = identity_service::launch().await?;

    Ok(())
}