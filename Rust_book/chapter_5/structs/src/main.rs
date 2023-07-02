fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    fn build_user(email: &str, username: &str) -> User {
        User {
            active: true,
            username: String::from(username),
            email: String::from(email),
            sign_in_count: 1,
        }
    }

    let mut user1 = User {
        active: true,
        username: String::from("Someuser123"),
        email: String::from("test@test.com"),
        sign_in_count: 1,
    };

    let mut user2 = build_user("test@5325.com", "Suckdick123");

    user1.email = String::from("test1234@test.com");
    user2.email = String::from("test1234@test.com");

    println!("User Email is { }", user1.email);
    println!("User2 Email is { }", user2.email);
    println!("{ } { } { } { }", user1.email, user1.active)
}
