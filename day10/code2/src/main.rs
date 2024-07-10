fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third1: &i32 = &v[2];
    println!("The third element is {}", third1);

    let third2: Option<&i32> = v.get(2);

    match third2 {
        Some(third1) => println!("The third element is {}", third1),
        None => println!("There is no third element."),
    }
}
