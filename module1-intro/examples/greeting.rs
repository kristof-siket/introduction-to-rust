fn main() {
    // Define a variable 'name' and assign it a string value
    let name = "Alice";

    // Print a friendly greeting using the 'name' variable
    println!("Hello, {}! Welcome to Rust.", name);

    // Create a list of fruits and print them one by one
    let fruits = ["Apple", "Banana", "Cherry", "Date"];
    
    for fruit in fruits.iter() {
        println!("I love {}s!", fruit);
    }
}
