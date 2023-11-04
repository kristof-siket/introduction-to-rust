fn main() {
    println!("Hello, world!");

    // function inside main
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    let five = add_one(4);
    println!("five is {}", five);

    // fn add_to_five(x: i32) -> i32 {
    //     x + five // Does not work because five is not in scope
    // }

    // Clojure: can capture the environment
    let add_to_five = |x: i32| -> i32 { x + five };

    let result = add_to_five(4);
    println!("result is {}", result);

    // Annotation is optional if the closure is used once
    let greet = |name| format!("Hello, {}!", name);

    let result = greet("Rust");
    println!("result is {}", result);

    // Without argument
    let ten = || 10;
    println!("ten is {}", ten());
}
