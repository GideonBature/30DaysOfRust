fn main() {
    let _a_num = 4;
    make_and_drop();
}

fn make_and_drop() {
    let a_box = Box::new(5);
    println!("{}", a_box);
}
