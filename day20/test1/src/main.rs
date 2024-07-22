fn main() {
    let mut n = 1;
    let b = Box::new(&mut n);

    **b += 1;
    println!("{}", n);
}
