// ! Borrwing

// * it's a key concept in Rust that allows you to use data without taking ownership of it
// * To accomplish this, Rust uses a borrowing mechanism. 
// * Instead of passing objects by value (T). objects can be passed by reference (&T).

// ! Borrowing is done through references, which come in two types:
// * 1- immutable (&T) 
// * 2- mutable (&mut T)
// This mechanism ensures safe memory access by controlling how values are accessed or modified.

// !! mmutable Borrowing
// An immutable reference allows you to read data without taking ownership.
//  Multiple immutable references can exist at the same time, but they cannot modify the data.

fn display(s:&String) {
    println!("{}",s);
}

fn main() {
    println!("Hello, Borrowing!");

    let s1 = String::from("immutable Borrowing");
    display(&s1); // * // Borrow `s1` immutably
    println!("S1 value is still valid {}",s1);
}
