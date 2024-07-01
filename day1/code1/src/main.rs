fn main() {
    let x = 5;
    let y = add_one(x);
    println!("The value of y is: {y}");
}

fn add_one(n: i32) -> i32 {
    n + 1
}
