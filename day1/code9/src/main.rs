fn get_second(vr: &Vec<i32>) -> i32 {
    vr[1]
}

fn main() {
    let mut v = vec![2, 4, 6];
    let n = get_second(&v);
    println!("{} {}", n, v[2]);
}
