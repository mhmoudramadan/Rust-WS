// RAII (Resource Acquisition Is Initialization)
// is a programming idiom that Rust embraces fully. 
// In Rust, RAII is implemented through the ownership system and the Drop trait. 
// RAII is central to its ownership and borrowing model. 
// Resources are automatically freed when the object goes out of scope, leveraging Rust’s drop mechanism.



// Basic Example

struct Resource;

/// The `impl Resource` block with the `fn new()->Resource` function is defining an associated function
/// named `new` for the `Resource` struct. This function serves as a constructor method for creating
/// instances of the `Resource` struct.
impl Resource {
    fn new()->Resource {
        println!("New resource acquired");
        Resource
    }
}

// Drop Trait
/// The `impl Drop for Resource {` block is implementing the `Drop` trait for the `Resource` struct in
/// Rust. By implementing the `Drop` trait, you define the behavior that should be executed when an
/// instance of the `Resource` struct goes out of scope and is being dropped.
impl Drop for Resource {
    fn drop(&mut self) {
        println!("Resource is released");
    }
}


fn main() {
    {
    let resource = Resource::new();
    println!("Hello, RAII!");
    } // Resource goes out of scope here, Drop is called

}
