// ? Ownership and moves 
// are fundamental concepts in Rust that ensure memory safety without a garbage collector


// ? Ownership dictates how data is managed, ensuring that resources (like memory) are freed when they are no longer needed.
// ! Understanding ownership and moves is crucial to writing safe and efficient Rust programs.



// !! Ownership
//*  Each value in Rust has a variable that’s called its owner.
//*  At any given time, there can be only one owner for a value. 
//*  When the owner goes out of scope, the value is dropped, and the memory is freed.
 
// !! Moves in Rust
// * When a value is moved, the ownership of the value is transferred from one variable to another. 
// * After a move the original variable can no longer be used because it no longer owns the data


// !! Deep Copy vs. Move (For Stack-only Data)
// * For types with a known size at compile time (like integers),
// * Rust automatically copies the value rather than moving it.


// !! Ownership with Functions
// * When passing data to a function, ownership is moved to the function 
// * unless the data is a copy type{Reference}.

fn take_ownership(s:String) {
    println!("{}",s);
}

fn main() {
    println!("Hello, Ownership and Moves!");

    // ! ownership
    {
        let my_str = String::from("Hello");
        println!("my string is {}",my_str);
    } // my_str goes out of scope and the memory for the String is freed

    // ! Move in rust 

    let s1 : &str = "mahmoud";
    // ?? before moved
     println!("{}", s1); 
    //  !! Move here
    let s2 = s1; // s1 is moved to s2

    // println!("{}", s1); // This would cause an error because s1 is no longer valid
    println!("{}", s2); // s2 now owns the String

    // ! Deep Copy

    let var1= 5;
    let var2 = var1; // ? var1 copied to var2
    println!("var1 is {} and var2 is {}",var1,var2);


    // ! Ownership with Functions
    let s3 = String::from("ownership with fun");
    take_ownership(s3); //s3 is moved to the function
    // println!("s3 is {}",s3); // Compile error
}
