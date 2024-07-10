fn main() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.25),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    println!("{:#?}", row[1]);
}
