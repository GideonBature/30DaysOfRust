fn main() {
    let x = 5;
    let a = 7;
    let y = Box::new(x);
    let z = Box::new(&a);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(7, **z);
}
