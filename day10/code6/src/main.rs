fn main() {
    let mut v = vec![100, 32, 57];

    for n_ref in &mut v {
        *n_ref += 50;

        println!("{n_ref}");
    }
}
