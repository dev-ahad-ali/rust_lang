fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        email: String::from("some@example.com"),
        username: String::from("username123"),
        sign_in_count: 1,
    };

    let user2 = User {
        // active: user1.active,
        // username: user1.username,
        // sign_in_count: user1.sign_in_count,
        email: String::from("new@email.com"),
        ..user1
    };

    user1.email = String::from("another@email.com");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
