// A library crate contains reusable code, typically in a lib.rs file.
// Library crates are designed to be used by other crates as dependencies.

// src/lib.rs
pub fn greet() {
    println!("Hello from the library crate!");
}


// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
