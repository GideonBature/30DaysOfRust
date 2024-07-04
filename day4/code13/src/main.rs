#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn square_perimeter(size: u32) -> Self {
        Self {
            width: 2 * size,
            height: 2 * size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);
    let sq_perimeter = Rectangle::square_perimeter(5);
    println!("{:#?}", sq);
    println!("{:#?}", sq_perimeter);
}
