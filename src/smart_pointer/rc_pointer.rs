use std::rc::Rc;

pub fn run_rc() -> () {
    // Create a Rc that owns the string.
    let data: Rc<String> = Rc::new(String::from("Hello, Rc!"));

    // Clone Rc to create an additional data owner.
    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data);

    println!("Data: {}", data);
    println!("Owner1: {}", owner1);
    println!("Owner2: {}", owner2);

    // Check the number of links.
    println!("Reference count: {}", Rc::strong_count(&data));
}