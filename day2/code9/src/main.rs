fn main() {
    let s = String::from("abcdefg");
    let s_slice = &s[2..5];

    println!("{} - {}", s, s_slice);
}
