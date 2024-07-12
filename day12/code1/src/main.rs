fn main() {
    let num_list = vec![34, 50, 25, 100, 65];

    let mut largest = &num_list[0];

    for num in &num_list {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number is {}", largest);
}
