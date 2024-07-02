fn main() {
    let name_string = String::from("Gideon Bature");

    let word1 = first_word(&name_string[0..10]);
    let word2 = first_word(&name_string[..]);
    
    let word3 = first_word(&name_string);

    let name_string_literal = "Gideon Bature";

    let word4 = first_word(&name_string_literal[0..10]);
    let word5 = first_word(&name_string_literal[..]);

    let word6 = first_word(name_string_literal);

    println!("{} {} {} {} {} {}", word1, word2, word3, word4, word5, word6);
}

fn first_word(n: &str) -> &str {
    let bytes = n.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &n[..i];
        }
    }
    &n[..]
}
