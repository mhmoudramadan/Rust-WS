/// converting to and from strings is commonly done using the ToString, FromStr, and Display traits
/// Rust automatically provides a ToString implementation for any type that implements the Display trait.
/// Display trait which automagically provides ToString and also allows printing the type as discussed in the section on print!.
/// 

use std::fmt;

#[derive(Debug)]
struct Person{
    name : String,
    age : u32,
}

/// This code snippet is implementing the `Display` trait for the `Person` struct in Rust. By
/// implementing the `Display` trait, the `fmt` function is defined for the `Person` struct. This
/// function specifies how a `Person` instance should be formatted when it is displayed, for example,
/// when using `println!` or `format!` macros.
/// 
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}


fn main(){

    let person = Person{
        name:String::from("Mahmoud"),
        age : 27,
    };

    println!("{}",person);
}