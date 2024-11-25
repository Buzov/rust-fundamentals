pub fn test_closures() {
    let square = |x| x * x;
    assert_eq!(square(5), 25);
}

pub fn test_fn_like_parameter() {
    fn plus_one(i: i32) -> i32 { i + 1 }

    let f = plus_one;
    let six = f(5);
    println!("{}", six);
    println!("fn_like_parameter result: {}", fn_like_parameter(f, 10));
    println!("fn_like_parameter result: {}", fn_like_parameter(f, 20));
}

fn fn_like_parameter(func: fn(i32) -> (i32), i: i32) -> i32 {
    func(i)
}