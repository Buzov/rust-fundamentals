pub fn test_vec() {
    for x in vec![1, 2, 3] {
        println!("x = {}", x);
    }

    let xs  = vec![1, 2, 3];
    for i in 0..xs.len() {
        let x = xs[i];
        println!("x = {}", x);
    }
    let xs2  = vec![1, 2, 3];
    print_slice(&xs2);
}

fn print_slice(xs: &[i32]) {
    for i in 0..xs.len() {
        println!("{}", xs[i]);
    }
}