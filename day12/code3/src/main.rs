fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
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

    let res = largest_i32(&num_list);
    println!("The largest number is {}", res);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let res = largest_char(&char_list);
    println!("The largest char is {}", res);
}
