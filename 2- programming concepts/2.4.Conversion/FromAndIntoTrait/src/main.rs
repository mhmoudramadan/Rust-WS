/// .  as known Primitive types can be converted to each other through casting.
///    Rust addresses conversion between custom types (i.e., struct and enum) by the use of traits.
///    The generic conversions will use the From and Into traits.
///    there are more specific ones for the more common cases 
/// 
/// 
///  
// The From and Into traits are inherently linked, and this is actually part of its implementation.
// If you are able to convert type A from type B
// then it should be easy to believe that we should be able to convert type B to type A.



/// From 
/// The From trait allows for a type to define how to create itself from another type,
/// providing a very simple mechanism for converting between several types
/// here are numerous implementations of this trait within the standard library for conversion of primitive and common types
/// 


#[derive(Debug)]
struct Number{
    value:u32,
}

/// impl keyword is used to define implementation blocks for structs, enums, or traits. 
/// It allows you to define methods associated with types, implement traits, and encapsulate behavior
///  Methods defined within an impl block can either be instance methods (which take &self, &mut self, or self as parameters)
///  or associated functions (which don't take self as a parameter).
///
/// This code snippet is implementing the `From` trait for converting a `u32` type into a custom type
/// `Number`.
impl std::convert::From<u32> for Number{

    fn from(item : u32) -> Self{
        Number{value : item}
    }
}



/// Into 
/// The Into trait is simply the reciprocal of the From trait
/// It defines how to convert a type into another type.
/// It is part of the standard library and provides a method called into() to perform conversions.
/// 
/// 
/// The code defines a struct NumberTag with a single field value of type u32 and implements the Into
/// trait to convert u32 into NumberTag.
/// 
/// Properties:
/// 
/// * `value`: The `NumberTag` struct has one field named `value` of type `u32`. The `Into` trait is
/// implemented for converting a `u32` value into a `NumberTag` instance. This allows you to create a
/// `NumberTag` instance from a `u32` value

#[derive(Debug)]
struct NumberTag{
    value:u32,
}

impl std::convert::Into<NumberTag> for u32{
    fn into(self) -> NumberTag{
        NumberTag { value : self}
    }
}



fn main() {
    
    /// from Implementation
    /// Easily Way  
    /// 
    let _my_str ="hello";

    let _my_string = String::from(_my_str);

    /// Using Own type Conversion
    /// 

    let _num = Number::from(300);
    println!("My number is {:?}", _num);


    /// into Implementation
    /// 
    /// 
    let var_ = 155;
    let num: NumberTag = var_.into();
    println!("the number is {:?}",num);
}
