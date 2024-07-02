fn main() {
    let mut s = String::from("Will Simon");

    let s_ref = &s;

    let s2 = *s_ref;

    println!("{}", s);
}
