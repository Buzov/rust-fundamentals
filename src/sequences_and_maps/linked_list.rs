use std::collections::LinkedList;

pub fn test_linked_list() {
    for x in vec![1, 2, 3] {
        println!("x = {}", x);
    }

    let xs  = vec![1, 2, 3];

    for i in 0..xs.len() {
        let x = xs[i];
        println!("x = {}", x);
    }

    let mut numbers: LinkedList<_> = xs.into_iter().collect();

    for (i, item) in numbers.iter().enumerate() {
        println!("i = {}, item = {}", i, item);
    }

    numbers.push_back(4);
    numbers.push_front(0);

    let xs2  = vec![1, 2, 3];
    print_slice(&xs2);
}

fn print_slice(xs: &[i32]) {
    for i in 0..xs.len() {
        println!("{}", xs[i]);
    }
}