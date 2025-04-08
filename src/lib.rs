#![allow(dead_code, unused_variables)]

pub mod database {
    pub mod models {
        pub enum Status {
            Connected,
            Interrupted,
        }
    }
    pub fn connect_to_database() -> models::Status {
        // connect to db...
        models::Status::Connected
    }
    pub fn get_user() {
        // fetch user from db and return
    }
}
pub mod auth_utils {
    pub struct Credentials {
        pub username: String,
        pub password: String,
    }
    pub fn login(cred: Credentials) {
        // login
        crate::database::get_user();
    }
}

pub fn authenticate(cred: auth_utils::Credentials) {
    if let crate::database::models::Status::Connected = database::connect_to_database() {
        auth_utils::login(cred);
    }
}
