use identity_service;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let _future = identity_service::launch().await?;

    Ok(())
}