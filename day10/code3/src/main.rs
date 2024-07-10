fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    
    println!("The first element is: {}", first);

    v.push(6);

    println!("The sixth element is: {}", &v[5]);

}
