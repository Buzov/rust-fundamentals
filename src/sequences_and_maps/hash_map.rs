fn main() {
    use std::collections::HashMap;

    // Create map
    let mut fruit_calories = HashMap::new();

    // Insert
    fruit_calories.insert("apple", 95);
    //fruit_calories.insert(10, 95);

    // Print
    println!("Apple calories: {}", fruit_calories["apple"]);
}