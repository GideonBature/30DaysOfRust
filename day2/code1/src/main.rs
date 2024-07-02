fn incr(n: &mut i32) {
    *n += 1;
}

fn main() {
    let mut n = 1;
    let m: &mut i32 = &mut n;
    incr(m);
    println!("{}", n);
}
