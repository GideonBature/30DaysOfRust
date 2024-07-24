use std::thread;

fn main() {
    let mut n = 1;

    let t = thread::spawn(move || {
        n = n + 1;

        thread::spawn(move || {
            n = n + 1;
        })
    });

    n = n + 1;

    t.join().unwrap().join().unwrap();
    println!("{}", n);
}
