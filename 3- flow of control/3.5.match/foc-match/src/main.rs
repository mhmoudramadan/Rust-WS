/// match keyword
/// used for pattern matching, providing a powerful control flow construct that can be used to match values
/// against patterns and run code based on which pattern is matched. 
/// It is similar to a switch statement in other languages, but much more powerful and flexible.


fn main() {

    // Basic syntax
    // 
    // . [ _ . ] pattern acts as a catch-all for any value not explicitly handled
    //  by other patterns (similar to the default case in a switch statement).
    
    let num =3u8;
    match num {
        1 => println!("number is 1"),
        2 => println!("number is 2"),
        3 => println!("number is 3"),
        _ => println!("number is 1"),
    }
    
}
