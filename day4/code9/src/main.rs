#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let a = area(&rect1);

    println!("{} x {} = {}", rect1.width, rect1.height, a);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
