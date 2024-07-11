use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    println!("{:?}", greeting_file_result);
}
