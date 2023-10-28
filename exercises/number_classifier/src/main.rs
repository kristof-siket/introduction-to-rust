use std::io::{self, Write};

fn main() {
    loop {
        prompt("Hello from the number classifier! Please enter a number: ");
        let mut input: String = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(num) => {
                if num % 2 == 0 {
                    println!("{} is even", num);
                } else {
                    println!("{} is odd", num);
                }
            }
            Err(_) => {
                println!("{} is not a number", input.trim());
            }
        }

        prompt("Another round? :) (y/n): ");

        input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "n" {
            break;
        }
    }

}

fn prompt(s: &str) {
    print!("{}", s);
    io::stdout().flush().expect("Failed to flush stdout")
}
