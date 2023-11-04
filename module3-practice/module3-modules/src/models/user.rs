
// Structs have an extra level of visibility with their fields
pub struct User {
    pub name: String,
    pub age: u8,
    pub email: String,
    pub is_active: bool,

    // This is not going to be visible when the struct is used externally
    something_private: String,
}

impl User {
    // "Constructor"
    pub fn new(name: String, age: u8, email: String, is_active: bool) -> User {
        User {
            name,
            age,
            email,
            is_active,
            something_private: String::from("This is private!"),
        }
    }

    pub fn get_something_private(&self) -> &String {
        &self.something_private
    }
}
