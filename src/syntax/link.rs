pub(super)fn links() {
    let mut x: i32 = 92;
    let r: &mut i32 = &mut 92; // explicit reference taking
    *r += 1; // explicit dereferencing of a reference
    println!("r: {}", r);
    let a = 5;
    let b = &a;
    println!("{}", b);
    println!("{}", a);
}