fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("red")).or_insert(70);

    for (k, v) in &scores {
        println!("{} - {}", k, v);
    }
}
