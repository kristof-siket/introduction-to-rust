use std::io::Write;

enum BookCategory {
    Fiction,
    NonFiction,
    Science
}

struct Book {
    title: String,
    author: String,
    category: BookCategory
}

const INVALID_BOOK_ERROR: &str = "Invalid book category";

fn main() {
    let book = Book {
        title: String::from("The Hitchhiker's Guide to the Galaxy"),
        author: String::from("Douglas Adams"),
        category: BookCategory::Science
    };

    let book2: Book = Book {
        title: String::from("Harry Potter and the Philosopher's Stone"),
        author: String::from("J. K. Rowling"),
        category: BookCategory::Fiction
    };

    let book3: Book = Book {
        title: String::from("The Selfish Gene"),
        author: String::from("Richard Dawkins"),
        category: BookCategory::NonFiction
    };

    for book in [book, book2, book3].iter() {
        match book.category {
            BookCategory::Fiction => print!("This is a fiction book!"),
            BookCategory::NonFiction => print!("This is a non-fiction book!"),
            BookCategory::Science => print!("This is a science book!"),
            _ => print!("{}", INVALID_BOOK_ERROR)
        }
    }

    println!();

    let title = prompt("Now, it is time for you to add your own book! Please enter the title: ");
    let author = prompt("Please enter the author: ");
    let category = prompt("Please enter the category (fiction, non-fiction, science): ");

    let category = match category.as_str() {
        "fiction" => BookCategory::Fiction,
        "non-fiction" => BookCategory::NonFiction,
        "science" => BookCategory::Science,
        _ => panic!("{}", INVALID_BOOK_ERROR)
    };

    let book = Book {
        title,
        author,
        category
    };

    println!("You have added the following book: {} by {}", book.title, book.author);
}

fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();

    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    return input.trim().to_string();
} 
