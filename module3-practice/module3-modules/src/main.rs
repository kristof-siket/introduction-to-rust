mod utils;
mod constants;
mod models;

fn main() {
    println!("Hello, world!");
    
    // Valid mad ops
    println!("1 + 2 = {}", utils::math::add(1, 2));
    println!("1 / 2 = {}", utils::math::divide(1, 2));


    // Divide by zero
    println!("1 / 0 = {}", utils::math::divide(1, 0));

    utils::messages::success_message();
    utils::messages::error_message();
    utils::messages::warning_message();
    utils::messages::info_message();

    println!("Link: {}", constants::links::GITHUB);
    println!("Error code: {}", constants::error_codes::PAGE_NOT_FOUND);

    // Create with public constructor
    let john: models::user::User = models::user::User::new(String::from("John"), 12, String::from("john@doe.com"), true);

    // Create the "regular" way -> not possible because there is a private field
    // let jane: models::user::User = models::user::User {
    //     name: String::from("Jane"),
    //     age: 13,
    //     email: String::from("jane@doe.com"),
    //     is_active: false,
    // };

    println!("User: {}", john.name);
    
    // Get private info with getter
    println!("User private info: {}", john.get_something_private());
}
