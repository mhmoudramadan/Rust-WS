// An attribute is metadata applied to some module, crate or item. This metadata can be used to/for:

// Key points

//1- conditional compilation of code
//2- set crate name, version and type (binary or library)
//3- disable lints (warnings)
//4- enable compiler features (macros, glob imports, etc.)
//5- link to a foreign library
//6- mark functions as unit tests
//7- mark functions that will be part of a benchmark
//8- attribute like macros


// Attributes look like 
    //1- #[outer_attribute] or
    //2- #![inner_attribute], 
    // with the difference between them being where they apply.

//     #[outer_attribute] applies to the item immediately following it. Some examples of items are: a function, 
//                         a module declaration, a constant, a structure, an enum.

//     #![inner_attribute] applies to the enclosing item (typically a module or a crate).
//         In other words this attribute is interpreted as applying to the entire scope in which it's placed.



// Attributes can take arguments with different syntaxes:

//     #[attribute = "value"]
//     #[attribute(key = "value")]
//     #[attribute(value)]

// Attributes can have multiple values and can be separated over multiple lines, too:

// #[attribute(value, value2)]

// #[attribute(value, value2, value3,
//             value4, value5)]



//  Outer Attribute

#[derive(Debug)]
struct Info {
    name : String,
    age:u32,
}



// Inner Attribute
// #![allow(unused_variables)]  --> if we need to use it like adding [!] should be placed at top of file
// else should be used without [!]

#[allow(unused_variables)]

fn main() {

let x:u32 = 30;
   
}
