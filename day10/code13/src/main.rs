fn main() {
    let mut v: Vec<i32> = vec![10, 2, 15];
    let mut _v2: Vec<&mut i32> = Vec::new();

    for i in &mut v {
        println!("{}", i);
    }
}
