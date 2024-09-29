// ? Ownership and moves 
// are fundamental concepts in Rust that ensure memory safety without a garbage collector


// ? Ownership dictates how data is managed, ensuring that resources (like memory) are freed when they are no longer needed.
// ! Understanding ownership and moves is crucial to writing safe and efficient Rust programs.




//* Each value in Rust has a variable that’s called its owner.
/// At any given time, there can be only one owner for a value. 
/// When the owner goes out of scope, the value is dropped, and the memory is freed.

fn main() {
    println!("Hello, Ownership and Moves!");

    // ! ownership
    {
        let my_str = String::from("Hello");
        println!("my string is {}",my_str);
    } // my_str goes out of scope and the memory for the String is freed
}
