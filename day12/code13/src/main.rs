struct Point<T> { x: T, y: T }

impl Point<i32> {
    fn f(&self) -> &i32 { &self.y }
}

impl<T> Point<T> {
    fn f(&self) -> &T { &self.x }
}

fn main() {
    let p: Point<i32> = Point { x: 1, y: 2 };
    println!("{}", p.f());
}
