fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<&mut i32> = Vec::new();

    for i in &mut v {
        v2.push(i);
    }
    *v2[0] = 5;

    let a = *v2[0];
    let b = v[0];

    println!("{} {}", a, b);
}
