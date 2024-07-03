struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        email: String::from("infoaboutgideon@gmail.com"),
        username: String::from("Bene"),
        sign_in_count: 1,
    };

    println!("{}", user1.email);
}
