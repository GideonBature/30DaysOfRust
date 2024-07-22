use std::ops::Deref;

#[derive(Clone, Copy)]
struct AccessLogger(i32);

impl Deref for AccessLogger {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        println!("deref");
        &self.0
    }
}

fn main() {
    let n = AccessLogger(-1);
    let x = *n + 1;
    let n2 = n;
    println!("{} {}", x, *n)
}
