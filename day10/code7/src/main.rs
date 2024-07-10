fn find_until(v: &Vec<i32>, n: i32, til: usize) -> Option<usize> {
    for i in 0 .. til {
        if v[i] == n {
            return Some(i);
        }
    }
    return None;
}

fn main() {
    let _result: Option<usize> = find_until(&vec![1, 2, 3], 4, 4);
/*
    match result {
        Some(value) => println!("This has a value"),
        None => println!("This has no value")
    }
*/
}
