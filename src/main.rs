fn main() {

    // Variable declaration without type annotation
    let age = 27;

    // Variable declaration with type annotation
    let weight: u32 = 80;

    // Variable declaration with type annotation and mutable
    let mut counter = 0; // 'counter' is mutable
    counter = 1; // We can change the value of 'counter' because it's mutable

    // Print number variables
    numbers();

    // Print float variables
    floats();

    // Pring char variables
    chars();

    // Print boolean variables
    booleans();

    println!("The counter is {}", counter);
    println!("My weight is {} kg.", weight);
    println!("I am {} years old.", age);
}

fn numbers() {
    // Signed integers
    let i8_min: i8 = i8::MIN;
    let i8_max: i8 = i8::MAX;
    println!("i8: Minimum = {}, Maximum = {}", i8_min, i8_max);

    let i16_min: i16 = i16::MIN;
    let i16_max: i16 = i16::MAX;
    println!("i16: Minimum = {}, Maximum = {}", i16_min, i16_max);

    let i32_min: i32 = i32::MIN;
    let i32_max: i32 = i32::MAX;
    println!("i32: Minimum = {}, Maximum = {}", i32_min, i32_max);

    let i64_min: i64 = i64::MIN;
    let i64_max: i64 = i64::MAX;
    println!("i64: Minimum = {}, Maximum = {}", i64_min, i64_max);

    let i128_min: i128 = i128::MIN;
    let i128_max: i128 = i128::MAX;
    println!("i128: Minimum = {}, Maximum = {}", i128_min, i128_max);

    // Unsigned integers
    let u8_min: u8 = u8::MIN;
    let u8_max: u8 = u8::MAX;
    println!("u8: Minimum = {}, Maximum = {}", u8_min, u8_max);

    let u16_min: u16 = u16::MIN;
    let u16_max: u16 = u16::MAX;
    println!("u16: Minimum = {}, Maximum = {}", u16_min, u16_max);

    let u32_min: u32 = u32::MIN;
    let u32_max: u32 = u32::MAX;
    println!("u32: Minimum = {}, Maximum = {}", u32_min, u32_max);

    let u64_min: u64 = u64::MIN;
    let u64_max: u64 = u64::MAX;
    println!("u64: Minimum = {}, Maximum = {}", u64_min, u64_max);

    let u128_min: u128 = u128::MIN;
    let u128_max: u128 = u128::MAX;
    println!("u128: Minimum = {}, Maximum = {}", u128_min, u128_max);

    // Platform-dependent integers
    let isize_min: isize = isize::MIN;
    let isize_max: isize = isize::MAX;
    println!("isize: Minimum = {}, Maximum = {}", isize_min, isize_max);

    let usize_min: usize = usize::MIN;
    let usize_max: usize = usize::MAX;
    println!("usize: Minimum = {}, Maximum = {}", usize_min, usize_max);
}

fn floats() {
    // Declaring and initializing f32 variables
    let temperature_celsius: f32 = 25.5;
    let pi_approximation: f32 = 3.14159;

    // Declaring and initializing f64 variables (the default for floating-point literals)
    let distance_miles: f64 = 1234.567;
    let weight_kilograms: f64 = 75.6;

    // Performing basic arithmetic operations
    let sum = temperature_celsius + 10.0; // Adding a constant
    let product = pi_approximation * 2.0; // Multiplying by a constant

    // Combining variables
    let total_distance = distance_miles + 100.0; // Adding a constant to a variable

    // Printing the results
    println!("Temperature in Celsius: {}", temperature_celsius);
    println!("Approximation of Pi: {}", pi_approximation);
    println!("Distance in Miles: {}", distance_miles);
    println!("Weight in Kilograms: {}", weight_kilograms);
    println!("Sum of Temperature and 10: {}", sum);
    println!("Product of Pi and 2: {}", product);
    println!("Total Distance: {}", total_distance);
}

fn operations() {
    // Addition
    let sum = 5 + 10;

    // Subtraction
    let difference = 95.5 - 4.3;

    // Multiplication
    let product = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;

    // Remainder
    let remainder = 43 % 5;

    // More complex expression
    let result = (4 * 30) as f32 + 56.7 / 32.2;

    // More complex expression, including variables and multiple operation types and paranthese
    let var_result: f32 = (sum as f32 + difference) as f32  * product as f32 / quotient % remainder as f32;

    // Integer division
    let integer_result = 5 / 2; // Results in 2 (integer division)
    let float_result = 5 as f32 / 2 as f32; // Results in 2.5 (casting to f32)

}

fn chars() {
    // Declaring and initializing a char variable
    let letter_a: char = 'A';
    let smiley_face: char = '😃';

    // Printing the results
    println!("Letter: {}", letter_a);
    println!("Smiley: {}", smiley_face);

    let letter: char = 'B';
    let digit: char = '1';

    println!("{} is a letter: {}", letter, letter.is_alphabetic());
    println!("{} is a digit: {}", digit, digit.is_digit(10));
}

fn booleans() {
    // Declaring and initializing a boolean variable
    let is_raining: bool = true;
    let is_sunny: bool = false;

    // Printing the results
    println!("Is it raining? {}", is_raining);
    println!("Is it sunny? {}", is_sunny);
}
