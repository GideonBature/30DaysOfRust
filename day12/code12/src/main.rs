fn print_slice<T: std::fmt::Debug>(v: &[T]) {
    for x in v {
        println!("{:?}", x);
    }
}

fn main() {
    print_slice(&[1, 2, 3]);
}
