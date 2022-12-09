#[get("/")]
pub fn get_identity() -> &'static str {
    "Hello, world!"
}