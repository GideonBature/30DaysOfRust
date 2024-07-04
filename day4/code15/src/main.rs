struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let mut r = Rectangle {
        width: 2,
        height: 6,
    };

    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);

    println!("area1: {}, area2: {}", area1, area2);

    r.set_width(4);
    
    let area3 = r.area();
    println!("area3: {}", area3);

    Rectangle::set_width(&mut r, 6);
    let area4 = Rectangle::area(&r);
    println!("area4: {}", area4);
}
