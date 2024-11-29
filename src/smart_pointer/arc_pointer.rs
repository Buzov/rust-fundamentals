use std::sync::Arc;
use std::thread::JoinHandle;

pub fn run_arc() -> () {
    // Create a Arc that owns the string.
    let data: Arc<String> = Arc::new(String::from("Hello, Arc!"));

    println!("\nArc Data: {}", data);
    // JoinHandle<(Vec<>>, ...)>
    let thread: JoinHandle<(Arc<String>, Arc<String>)> = std::thread::spawn(move || -> (Arc<String>, Arc<String>) {
        let owner1 = Arc::clone(&data);
        let owner2 = Arc::clone(&data);
        (owner1, owner2)
    });
    // Clone Arc to create an additional data owner.

    let (owner1, owner2) = thread.join().unwrap();

    println!("Owner Arc 1: {}", owner1);
    println!("Owner Arc 2: {}", owner2);

    let data_b: Arc<String> = Arc::clone(&owner1);
    // Check the number of links.
    println!("Reference count Arc: {}", Arc::strong_count(&data_b));

    // std::mem::drop(owner1)
    drop(owner1);

    println!("Reference count Arc: {}", Arc::strong_count(&data_b));
}