fn main() {
    let xs: [u8; 3] = [1, 2, 3];
    assert_eq!(xs[0], 1); // index -- usize
    assert_eq!(xs.len(), 3); // len() -- usize
    let mut buf = [0u8; 1024]; // a 1024 element buffer filled with zeros


    // Create vector
    let mut fruits = vec!["apple", "banana", "cherry"];

    // Add element
    fruits.push("orange");

    // Print
    println!("Fruit vector: {:?}", fruits);
}