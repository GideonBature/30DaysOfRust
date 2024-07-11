#![allow(unused)]
fn main() {
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("file.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }
}
