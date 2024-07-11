#![allow(unused)]
fn main() {
    use std::fs;
    use std::io;

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("myInfo.txt")
    }
}
