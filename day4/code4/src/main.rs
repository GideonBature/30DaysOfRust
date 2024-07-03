struct User {
    name: String,
    age: u32,
    email: String,
    is_new: bool,
}

fn build_user(name: String, age: u32, email: String, is_new: bool) -> User {
    User {
        name,
        age,
        email,
        is_new,
    }
}

fn main() {
    let mut user1 = build_user (
        String::from("Grace Andrew"),
        29,
        String::from("andrewgrace438@gmail.com"),
        true
        );

    println!("my name is {}, I am {} years old!", user1.name, user1.age);
}
