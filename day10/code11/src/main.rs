fn main() {
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        v.push(*i);
    }

    println!("{} {} {}", v[2], v[4], v[5]);
}
