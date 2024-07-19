fn main() {
    let example_closure = || 5;

    let s = example_closure();

    println!("Number: {}", s);
}
