// Visible for other modules (can be used externally)
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        handle_error("Cannot divide by zero!");
        return 0;
    }

    a / b
}

// Not visible for other modules (can be used internally only)
fn handle_error(msg: &str) {
    println!("Error: {}", msg);
}
