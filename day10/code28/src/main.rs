use std::collections::HashMap;

fn main() {
    let mut h = HashMap::new();

    h.insert("k1", 0);
    let v1 = h["k1"];

    h.insert("k2", 1);
    let v2 = h["k2"];

    println!("{} - {}", v1, v2);
}
