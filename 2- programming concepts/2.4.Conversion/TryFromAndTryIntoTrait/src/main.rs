/// traits that allow for fallible conversions between types, meaning conversions that can fail
/// TryFrom and TryInto are generic traits for converting between types.
/// the TryFrom/TryInto traits are used for fallible conversions, and as such, return Results. 


/// Tryfrom  Guide
///    
// pub trait TryFrom<T>: Sized {
//     type Error;

//     fn try_from(value: T) -> Result<Self, Self::Error>;
// }

// T: The type you want to convert from.
// Self: The type you are converting to.
// Error: The type returned in case of a failed conversion.
// The method try_from returns a Result that is Ok(Self) on success and Err(Self::Error) on failure.

use std::convert::TryFrom;

#[derive(Debug)]
struct Age_(u8);

impl TryFrom<i32> for Age_ {

    type Error = String;
    fn try_from(value : i32) -> Result<Self,Self::Error>{
        if value >= 0 && value <= 120 {
            Ok(Age_(value as u8))
        }else{
            Err(String::from("Age Invalid"))
        }
    }
    
}


/// 
/// The TryInto trait is automatically implemented for any type that implements the TryFrom trait.
/// pub trait TryInto<T>: Sized {
/// type Error;
/// 
/// fn try_into(self) -> Result<T, Self::Error>;
/// }
// T: The type to which you want to convert the value.
// self: The value being converted.
// The method try_into(self) returns a Result type, which is either Ok(T) if the conversion succeeds or Err(Self::Error) if it fails.

use std::convert::TryInto;


fn main() {

    // try_from
    let age_ = Age_::try_from(27);
    println!("Age is {:?}",age_);

    let invalid_age_ = Age_::try_from(130);
    println!("invalid age is {:?}",invalid_age_);

    // try_into
    let number_ = 30i32;
    let my_age:Result<Age_,String> = number_.try_into();

    match my_age{
        Ok(a) => println!("Age is valid: {:?}", a),
        Err(e) => println!("Error: {}", e),
    }
    
}