use std::io;

fn main() {
    let num1 = 2;

    let num2 = 3;

    println!("{} + {} = {}", num1, num2, num1 + num2);   

    // Input the user for a number
    println!("Please input a number: ");

    // Create a variable to store the user input
    let mut input: String = String::new();

    // Read the user input and store it in the variable
    let _ = io::stdin().read_line(&mut input);
    // Convert the user input to a number
    let input: i32 = input.trim().parse().unwrap();

    // Print the result
    println!("{} + {} = {}", num1, input, num1 + input);
}
