use rust_lang_book::{authenticate, database::models::Credentials};

fn main() {
    let cred = Credentials {
        username: String::from("rustacean"),
        password: String::from("admin123"),
    };

    authenticate(cred);
}
