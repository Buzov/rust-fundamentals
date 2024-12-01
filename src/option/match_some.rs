fn divide(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None // Return None if division by zero
    } else {
        Some(dividend / divisor) // Return the result wrapped in Some
    }
}

fn test_option() {
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);

    // Using pattern matching
    match result1 {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by zero"),
    }

    // Using `if let` for concise handling
    if let Some(value) = result2 {
        println!("Result: {}", value);
    } else {
        println!("Cannot divide by zero");
    }

    // Using `.unwrap_or` to provide a default value
    let default_value = result2.unwrap_or(0.0);
    println!("Default value for invalid operation: {}", default_value);
}