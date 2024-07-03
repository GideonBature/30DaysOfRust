struct User {
    name: String,
    age: u32,
    email: String,
    is_active: bool,
}

fn build_user(name: String, age: u32, email: String, is_active: bool) -> User {
    User {
        name,
        age,
        email,
        is_active,
    }
}

fn main() {
    let user1 = build_user(
        String::from("Grace Andrew"),
        29,
        String::from("gideonbature5@gmail.com"),
        true
        );

    let user2 = User {
        email: String::from("graceandrewajogi@yahoo.com"),
        ..user1
    };
    println!("{} {} {} {}", user2.email, user1.email, user2.age, user1.age);
}
