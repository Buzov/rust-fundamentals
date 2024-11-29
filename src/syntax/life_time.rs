use crate::types::tuple::tuple;

pub(super) fn test_life_time_in_fn() {
    let x = 1;
    let r: &i32;
    {
        let y = 2;
        r = life_time_in_fn(&x, &y); // ok
    }
    println!("{}", *r);
    //    let x = 1;
    //    let r: &i32;
    //    let z: i32;
    //    {
    //        let y = 2;
    //        r = &y; // borrowed value does not live long enough
    //        z = y;
    //    }
    //    println!("{}", z);
    //    println!("{}", *r);
}

fn life_time_in_fn<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    println!("life_time_in_fn y: {}", y);
    // y если вернуть y - будет ошибка компиляции
    x // parameter and the return type are declared
    // with different lifetimes
}
pub(super) fn test_fn_tuple() {
    let (x, y) = fn_tuple();
    println!("x: {}, y: {}", x, y);
    tuple();
}

fn fn_tuple<'a>() -> (i32, &'a str) {
    (1, "dfgd")
}

fn tuple_fn(fun: &dyn Fn() -> (f64, f64)) -> (f64, f64) {
    fun()
}