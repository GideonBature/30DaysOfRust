fn main() {
    let a = [2, 4, 6, 8, 10, 12];

    let slice = &a[2..4];

    println!("{}", *slice);

    assert_eq!(slice, &[6, 8]);
}
