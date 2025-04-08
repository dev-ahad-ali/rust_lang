#![allow(dead_code, unused_variables)]

pub mod auth_utils;
pub mod database;

use auth_utils::login;
use database::{connect_to_database, models::Status};

pub fn authenticate(cred: auth_utils::Credentials) {
    if let Status::Connected = connect_to_database() {
        login(cred);
    }
}
