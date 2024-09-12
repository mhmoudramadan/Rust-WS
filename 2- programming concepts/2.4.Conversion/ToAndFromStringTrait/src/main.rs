/// converting to and from strings is commonly done using the ToString, FromStr, and Display traits
/// Rust automatically provides a ToString implementation for any type that implements the Display trait.
/// Display trait which automagically provides ToString and also allows printing the type as discussed in the section on print!.
/// 

// You can use to_string() on built-in types like integers, floats, and booleans because they implement Display.
// The FromStr trait is implemented for many types, 
// allowing you to easily parse strings into integers, floats, and more.


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

    //  on built-in types to_string
    let another_person = person.to_string();
    println!("{}",another_person);


    // Parse String
    // idiomatic approach to this is to use the [parse] function and either to arrange for type inference 
    // or to specify the type to parse using the 'turbofish' syntax. 

    let new_number:i16 = "300".parse().unwrap();
    let new_2  = "10000".parse::<i32>().expect("Not a Valid Number");
    let new_3  = "100000".parse::<i32>().unwrap();
    println!("Parsed Number is {}",new_number);
    println!("Parsed Number is {}",new_2);
    println!("Parsed Number is {}",new_3);


    // FromStr Trait

    // FromStr Trait:
    // The FromStr trait is used to define how to parse a string and convert it into a specific type. 
    // The method returns Result<Self, Self::Err>, where Err is the error type in case the parsing fails.
    
    // pub trait FromStr : Sized {
    // type Error;
    // fn from_str(s: &str) -> Result<Self, Self::Err>  

}