struct Student {
    name: String,
    email: String,
    is_present: bool,
    age: u32,
}

fn main() {
    let mut student1 = Student {
        name: String::from("Gideon Bature"),
        email: String::from("gideonbature5@gmail.com"),
        is_present: true,
        age: 95,
    };

    println!("My name is {}, I am {} old, and it's {} that I am in school today!", student1.name, student1.age, student1.is_present);

    student1.name = String::from("Grace Andrew Ajogi");
    
    println!("My name is {}, I am {} old, and it's {} that I am in school today!", student1.name, student1.age, student1.is_present);
}
