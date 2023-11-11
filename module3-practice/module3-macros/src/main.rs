// Use macros
fn main() {
    // Macros are called with a bang !
    println!("Hello, world!"); // This we have seen before

    // vec! is a macro that creates a new vector
    let v = vec![1, 2, 3];

    // assert! is a macro that checks that its argument is true
    assert!(v.len() == 3); // Good for testing

    // format! is a macro that creates a String
    let s = format!("Hello, world!"); // Similar to println! but returns a String

    // env! is a macro that looks up an environment variable at compile time
    let path = env!("PATH");
    println!("PATH: {}", path);

    let a_b = concat!("A", "B"); // Concatenate literals at compile time
    println!("a_b: {}", a_b);

    // panic! is a macro that crashes the program with a message
    panic!("crash and burn");
}


