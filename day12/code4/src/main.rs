fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let num_list = vec![34, 50, 25, 100, 65];

    let result = largest(&num_list);
    println!("The largest number is {}", result);

    let char_list = vec!['G', 'i', 'd', 'e', 'o', 'n'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
