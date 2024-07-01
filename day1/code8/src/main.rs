fn main() {
    let x = Box::new(20);
    let y = Box::new(&x);
    let z = &x;

    println!("{}", **z);
}
