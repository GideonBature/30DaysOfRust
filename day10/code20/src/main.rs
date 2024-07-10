fn main() {
    let name = String::from("Grace Andrew Ajogi");

    for ch in name.chars() {
        println!("{}", ch);
    }

    for ch_byte in name.bytes() {
        println!("{}", ch_byte);
    }
}
