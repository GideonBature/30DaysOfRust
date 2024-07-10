fn main() {
    let v = vec![100, 32, 57];

    for n_ref in &v {
        println!("{n_ref}");
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }
}
