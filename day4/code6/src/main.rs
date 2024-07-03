struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(8, 16 ,32);
    let origin = Point(2, 4, 6);

    println!("{} - {} - {} - {} - {} - {}", black.0, black.1, black.2, origin.0, origin.1, origin.2);
}
