pub struct Credentials {
    pub username: String,
    pub password: String,
}
pub fn login(cred: Credentials) {
    // login
    crate::database::get_user();
}
