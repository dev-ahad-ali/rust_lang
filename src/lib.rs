#![allow(dead_code, unused_variables)]
pub enum Status {
    Connected,
    Interrupted,
}

pub mod database {
    pub fn connect_to_database() -> crate::Status {
        // connect to db...
        crate::Status::Connected
    }
    pub fn get_user() {
        // fetch user from db and return
    }
    pub mod models {
        pub struct Credentials {
            pub username: String,
            pub password: String,
        }
    }
}
mod auth_utils {
    fn login(cred: crate::database::models::Credentials) {
        // login
        crate::database::get_user();
    }
}

pub fn authenticate(cred: crate::database::models::Credentials) {
    if let Status::Connected = database::connect_to_database() {}
}
