#![allow(dead_code)]

// Main entry point for the program
fn main() {
    greet_simple();
    greet("Kristof");
    println!("1 + 2 = {}", add(1, 2));

    // Destructure the tuple
    let (sum, difference) = add_substract(1, 2);
    println!("1 + 2 = {}, 1 - 2 = {}", sum, difference);

    // Create a dog with the new() associated function (factory method)
    let my_dog: Dog = Dog::new("Rex", 5);

    // Call the bark() method on the dog (instance-level)
    my_dog.bark();

    // Of course, we can still create a dog the old way
    let mut my_dog2: Dog = Dog {
        name: "Bobby".to_string(),
        age: 5,
    };

    // Call the bark() method on the dog (instance-level)
    my_dog2.bark();

    // Destructure my_dog2
    let new_age = my_dog2.grow(2);

    // Call the grow() method on the dog (instance-level), which returns a value and modifes the dog
    println!("{} is now {} years old", my_dog2.name, new_age);

    // The inner state of my_dog2 has been modified
    println!("{} is now {} years old", my_dog2.name, my_dog2.age);

    // Create a color
    let color = Color::Red;

    // Call the to_hex() method on the color (instance-level)
    println!("{} is {}", color.to_string(), color.to_hex());
}

// Function without an argument (optional -> ())
fn greet_simple() -> () {
    println!("Hello, world!");
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function with return value
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Function with multiple return values
fn add_substract(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b)
}

// Struct
struct Dog {
    name: String,
    age: u8,
}

// Associated function & method
impl Dog {
    // This is an associated function (called on the type)
    fn new(name: &str, age: u8) -> Dog {
        Dog {
            name: name.to_string(),
            age,
        }
    }

    // This is a method (called on an instance of the type)
    fn bark(&self) {
        println!("{} barks!", self.name);
    }

    // A method with argument and return value
    // This requires the caller object to be mutable!
    fn grow(&mut self, years: u8) -> u8 {
        self.age += years;
        self.age
    }
}

// Enum
enum Color {
    Red,
    Green,
    Blue,
}

// Enum with function
impl Color {
    fn to_hex(&self) -> String {
        match self {
            Color::Red => "#FF0000".to_string(),
            Color::Green => "#00FF00".to_string(),
            Color::Blue => "#0000FF".to_string(),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Color::Red => "Red".to_string(),
            Color::Green => "Green".to_string(),
            Color::Blue => "Blue".to_string(),
        }
    }
}
