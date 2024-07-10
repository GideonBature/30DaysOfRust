fn main() {
    let v = vec![100, 32, 57];

    for n_ref in &v {
        let n_result = *n_ref + 2;
        println!("{n_result}");
    }
}
