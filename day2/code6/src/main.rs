#![allow(unused)]
fn main() {
    type Document = Vec<String>;

    fn new_document(words: Vec<String>) -> Document {
        words
    }

    fn add_word(this: &mut Document, word: String) {
        this.push(word);
    }

    fn get_words(this: &Document) -> &[String] {
        this.as_slice()
    }
}
