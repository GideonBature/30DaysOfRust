fn main() {
    let mut v = vec![String::from("Hello ")];
    let s = &mut v[0];

    s.push_str("world");
    println!("{s}");
}
