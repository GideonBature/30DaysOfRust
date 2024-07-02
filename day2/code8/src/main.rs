fn main() {
    let mut a_num = 5;
    inner(&mut a_num);
    println!("{}", a_num);
}

fn inner(x: &mut i32) {
    let another_num = 1;
    let a_stack_ref = &another_num;

    let a_box = Box::new(8);
    let a_box_stack_ref = &a_box;
    let a_box_heap_ref = &*a_box;

    *x += 20;

    println!("{}", *x);
}
