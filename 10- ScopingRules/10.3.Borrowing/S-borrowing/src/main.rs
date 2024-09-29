// ! Borrwing

// * it's a key concept in Rust that allows you to use data without taking ownership of it
// * To accomplish this, Rust uses a borrowing mechanism. 
// * Instead of passing objects by value (T). objects can be passed by reference (&T).

// ! Borrowing is done through references, which come in two types:
// * 1- immutable (&T) 
// * 2- mutable (&mut T)
// This mechanism ensures safe memory access by controlling how values are accessed or modified.

// !! immutable Borrowing
// An immutable reference allows you to read data without taking ownership.
//  Multiple immutable references can exist at the same time, but they cannot modify the data.

fn display(s:&String) {
    println!("{}",s);
}

// !! Mutable Borrowing
// * A mutable reference allows you to modify the borrowed value. 
// * only one mutable reference can exist at a time to prevent data races and ensure memory safety.

fn modify_value(s:&mut String){
    s.push_str(".borrow");
}


// !! Restriction on Mutable Borrowing
// *  Rust enforces strict borrowing rules:
// * You cannot have both mutable and immutable references to a variable at the same time.
// * This ensures that no other part of the code can access the data while it's being modified, preventing data races.



// !!Borrowing and Function Scopes
// * References are only valid within the scope of the function they are borrowed.
// * This means mutable and immutable borrows must stay within their valid scope.



// !! Using References with Slices
// *  Slices allow you to borrow a part of a data structure. String slices (&str) and array slices are common examples of borrowing.

fn get_first_word(s: &String)-> &str {
    let bytes= s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i] // return a slice of the string
        }
    }
    &s[..] // *  Return the whole string if no space is found
}

fn main() {
    println!("Hello, Borrowing!");

    // ! immutable Borrowing
    let s1 = String::from("immutable Borrowing");
    display(&s1); // * // Borrow `s1` immutably
    println!("S1 value is still valid {}",s1);


    // ! mutable Borrowing
    let mut s2 = String::from("mutable ");
    modify_value(&mut s2);
    println!("modified valued after mutable borrowing is {}",s2);


    // ! Restriction on Mutable Borrowing
    let mut s = String::from("Restriction borrow");

    let r1 = &s; // Immutable borrow
    let r2 = &s; // Another immutable borrow

    // let r3 = &mut s; // Error: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{} and {}", r1, r2);


    // ! Borrow and fun scope 
    let mut s4 = String::from("hello");

    {
        let r1 = &mut s4; // Mutable borrow in inner scope
        r1.push_str(", Borrowing!");
    } // r1 goes out of scope, mutable borrow ends

    let r5 = &s4; // Now we can borrow immutably
    println!("String after modification: {}", r5);

    // ! Using References with Slices

    let slice_str = String::from("Slices From Reference");

    let word = get_first_word(&slice_str); // ? Borrow string
    println!("First word: {}", word);

}


