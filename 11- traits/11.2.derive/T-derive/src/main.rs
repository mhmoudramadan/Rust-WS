// !! derive trait
// * The compiler is capable of providing basic implementations for some traits via the #[derive] attribute. 
// * These traits can still be manually implemented if a more complex behavior is required.
// the derive attribute allows you to automatically implement common traits for your structs and enums. This reduces the need to write repetitive boilerplate code.
//  The most common traits that can be automatically derived 
// include 
// * Debug, 
// * Clone, 
// * PartialEq,
// * Eq, 
// * PartialOrd
// * Ord
// * Hash
// * and Default.
fn main() {
    println!("Hello, derive trait!");
}
