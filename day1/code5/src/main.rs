fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");
    greet(&m1, &m2);
    println!("{} {}", m1, m2);

}

fn greet(m1: &String, m2: &String) {
    println!("{} {}!", m1, m2);
}
