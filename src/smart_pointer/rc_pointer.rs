use std::rc::Rc;

// In Rust, Rc (reference counting) is a smart pointer that enables shared ownership of data with
// reference counting. It is useful when multiple parts of your code need to own the same data,
// and the data is immutable.
//
// Key Characteristics of Rc:
//
// Used for shared ownership.
// Works only in single-threaded applications (for multithreaded scenarios, use Arc).
// The data referenced by Rc is immutable by default. To enable mutability, you can use Rc<RefCell<T>>.
//
// When to Use Rc?
//
// When you need shared ownership of immutable data in a single-threaded context.
// For complex data structures, such as graphs or trees, where nodes have multiple parents.
// When mutability of data is needed within Rc, you can use Rc<RefCell<T>>.
//
// When NOT to Use Rc?
//
// In multithreaded applications (use Arc instead for thread-safe shared ownership).
// If ownership can be expressed with references or Box, as these are simpler and faster.
pub fn run_rc() -> () {
    // Create a Rc that owns the string.
    let data: Rc<String> = Rc::new(String::from("Hello, Rc!"));

    // Clone Rc to create an additional data owner.
    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data);

    println!("\nRc Data: {}", data);
    println!("Owner Rc 1: {}", owner1);
    println!("Owner Rc 2: {}", owner2);

    // Check the number of links.
    println!("Reference count Rc: {}", Rc::strong_count(&data));

    std::mem::drop(owner1);

    println!("Reference count Rc: {}", Rc::strong_count(&data));
}