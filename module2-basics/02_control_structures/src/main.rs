fn main() {

    // Inputs
    let is_raining: bool = false;
    let is_rain_forecasted: bool = false;
    let cloud_color_tone: &str = "dark";


    // Logical expression with != operator
    let is_cloud_not_dark: bool = cloud_color_tone != "dark";

    // Simplified logic of bring_umbrella
    let bring_umbrella: bool = is_raining || is_rain_forecasted || cloud_color_tone == "dark";


    // Comparison operators and if-else
    let mut temperature: i32 = 25; // Current temperature in degrees Celsius

    if temperature > 30 {
        println!("It's a hot day!"); // If temperature is greater than 30°C
    } else if temperature >= 20 {
        println!("It's a pleasant day!"); // If temperature is between 20°C and 30°C (inclusive)
    } else {
        println!("It's a cold day!"); // If temperature is less than 20°C
    }
    

    // Pattern matching
    temperature = 25; // Current temperature in degrees Celsius

    match temperature {
        30 => println!("It's a hot day!"), // If temperature is 30°C
        20..=29 => println!("It's a pleasant day!"), // If temperature is between 20°C and 29°C (inclusive)
        _ => println!("It's a cold day!"), // If temperature is less than 20°C
    }

    print!("Bring umbrella: {}", bring_umbrella);

    patterns();
    loops();
}

fn patterns() {

    // Literal patterns
    let day: &str = "Monday";

    match day {
        "Monday" => {
            println!("It's the start of the week!");
        }
        "Friday" => {
            println!("It's the end of the week!");
        }
        _ => {
            println!("It's an ordinary day.");
        }
    }

    // Range patterns
    let temperature: i32 = 25;

    match temperature {
        0..=10 => {
            println!("It's very cold.");
        }
        11..=20 => {
            println!("It's chilly.");
        }
        21..=30 => {
            println!("It's comfortable.");
        }
        _ => {
            println!("It's warm.");
        }
    }

    // Tuple patterns
    let coordinates: (i32, i32) = (3, 4);

    match coordinates {
        (0, 0) => {
            println!("You're at the origin.");
        }
        (x, 0) => {
            println!("You're on the x-axis at position {}.", x);
        }
        (0, y) => {
            println!("You're on the y-axis at position {}.", y);
        }
        (x, y) => {
            println!("You're at coordinates ({}, {}).", x, y);
        }
    }

    // Enum patterns
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    let light: TrafficLight = TrafficLight::Red;

    match light {
        TrafficLight::Red => {
            println!("Stop!");
        }
        TrafficLight::Yellow => {
            println!("Slow down!");
        }
        TrafficLight::Green => {
            println!("Go!");
        }
    }

    // Optional values
    let age: Option<i32> = Some(20);

    match age {
        Some(age) => {
            println!("Age is {}.", age);
        }
        None => {
            println!("Age is unknown.");
        }
    }
}

fn loops() {
    // Loop loop
    let mut count = 0;

    loop {
        println!("Hello, World!");
        count += 1;

        if count >= 5 {
            break; // Exit the loop after printing "Hello, World!" 5 times
        }
    }

    // For loop
    let fruits = ["apple", "banana", "cherry", "date", "elderberry"];

    for fruit in fruits {
        println!("I like {}!", fruit);
    }

    // For loop with range
    for number in 1..=5 {
        println!("Number {}!", number);
    }

    // While loop
    let mut temperature = 25;

    while temperature > 20 {
        println!("It's still warm outside with {}°C.", temperature);
        temperature -= 2;
    }

    println!("It's getting cooler now, it is {} °C.", temperature);

    // Returning from loops
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2; // Break the loop and return count * 2
        }
    };

    println!("The result is {}.", result);

    // Nesting loops
    for i in 1..=3 {
        for j in 1..=3 {
            println!("({}, {})", i, j);
        }
    }

    // Breaking nested loops
    for i in 1..=3 {
        for j in 1..=3 {
            println!("({}, {})", i, j);

            if i == 2 && j == 2 {
                break; // Breaks only from the inner loop
            }
        }
    }

    // Breaking nested loops with loop labels
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            println!("({}, {})", i, j);

            if i == 2 && j == 2 {
                break 'outer; // Breaks from the outer loop
            }
        }
    }
}
