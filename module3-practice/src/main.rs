#![allow(dead_code)]

fn main() {
    // Named tuple
    struct IntPair(i32, i32);

    // Create two instances of the struct
    let pair: IntPair = IntPair(1, 2);
    let pair2: IntPair = IntPair(2, 3);

    // Destructure the struct and get the values
    let IntPair(x1, y1) = pair;

    // Same with unnamed tuple
    let u_pair: (i32, i32) = (1, 2);
    let u_pair2: (i32, i32) = (2, 3);

    // Struct with named fields
    struct Dog {
        name: String,
        age: u8,
    }

    let dog: Dog = Dog {
        name: String::from("Rusty"),
        age: 5,
    };

    // Access the fields
    println!("{} is {} years old", dog.name, dog.age);

    // Alternatively, accessing with destructuring
    let Dog { name, age } = dog;
    println!("{} is {} years old", name, age);

    // Unit struct
    struct UnitStruct;

    // Enum
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let my_up: Direction = Direction::Up;

    // Enum with data
    enum OptionalInt {
        Value(i32),
        Missing,
    }

    let x: OptionalInt = OptionalInt::Value(5);
    let y: OptionalInt = OptionalInt::Missing;

    // Enum with classic struct
    enum Shape {
        Rectangle { width: u32, height: u32 },
        Square(u32),
        Circle(f64),
    }

    let rect: Shape = Shape::Rectangle {
        width: 10,
        height: 20,
    };

    match my_up {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }

    // Enum as int32
    enum Color {
        Red = 10,
        Green = 20,
        Blue = 30,
    }

    println!("Red as i32: {}", Color::Red as i32);
    println!("Green as i32: {}", Color::Green as i32);
    println!("Blue as i32: {}", Color::Blue as i32);
}
