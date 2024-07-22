use std::rc::Rc;

struct Example;
impl Drop for Example {
    fn drop(&mut self) {
        println!("drop");
    }
}

fn main() {
    let x = Rc::new(Example);
    let y = Rc::clone(&x);

    println!("A");
    drop(x);
    println!("B");
    drop(y);
    println!("C");
}
