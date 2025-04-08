use rust_lang_book::{auth_utils::Credentials, authenticate};

fn main() {
    let cred: Credentials = Credentials {
        username: String::from("rustacean"),
        password: String::from("admin123"),
    };

    authenticate(cred);
}
