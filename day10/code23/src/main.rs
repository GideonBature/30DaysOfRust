fn main() {
    use std::collections::HashMap;

    let mut ages = HashMap::new();

    ages.insert(String::from("Gideon Bature"), 17);
    ages.insert(String::from("Grace Andrew"), 15);

    let name = String::from("Grace Andrew");
    let age = ages.get(&name).copied().unwrap_or(0);

    println!("{}", age);
}
