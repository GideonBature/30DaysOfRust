fn main() {
    println!("&Sring={} &str={}", std::mem::size_of::<&String>(), std::mem::size_of::<&str>());
}
