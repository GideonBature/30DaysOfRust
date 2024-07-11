fn remove_zeros(v: &mut Vec<i32>) {
    for (i, t) in v.clone().iter().enumerate().rev() {
        if *t == 0 {
            v.remove(i);
            v.shrink_to_fit();
        }
    }
}

fn reverse(v: &mut Vec<String>) {
    let n = v.len();
    for i in 0 .. n / 2 {
        let s1 = v[i].clone();
        let s2 = v[n - i - 1].clone();
        v[i] = s2;
        v[n - i - 1] = s1;
    }
}

fn main() {
    let mut v: Vec<i32> = vec![1, 0,  2, 0, 3, 0, 4, 0, 5];

    let mut x: Vec<String> = vec![String::from("Gideon"), String::from("Bature")];

    remove_zeros(&mut v);

    println!("{:?}", v);

    reverse(&mut x);

    println!("{:?}", x);
}
