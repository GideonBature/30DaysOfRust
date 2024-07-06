#[derive(Debug)]
enum Either {
    Left(usize),
    Right(String)
}

fn main() {
    let x = Either::Right(String::from("Grace Andrew Ajogi"));
    let value = match x {
        Either::Left(n) => n,
        Either::Right(s) => s.len()
    };
    println!("{x:?} {value}");
}
