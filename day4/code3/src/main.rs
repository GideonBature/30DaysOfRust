struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user (
        String::from("graceandrewajogi@gmail.com"),
        String::from("Grace Andrew"),
    );

    println!("{}", user1.username);
}
