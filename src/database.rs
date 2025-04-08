pub mod models;
pub fn connect_to_database() -> models::Status {
    // connect to db...
    models::Status::Connected
}
pub fn get_user() {
    // fetch user from db and return
}
