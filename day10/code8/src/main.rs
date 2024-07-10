fn main() {
    let mut v = Vec::new();
    let s = String::from("Hello ");

    v.push(s);
    v[0].push_str("world");

    println!("original: {}", s);
    println!("new: {}", v[0]);
}
