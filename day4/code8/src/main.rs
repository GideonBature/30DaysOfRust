#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale: u32 = 2;

    let rect1 = Rectangle {
        width: dbg!(10 * scale),
        height: 15
    };

    dbg!(&rect1);
}
