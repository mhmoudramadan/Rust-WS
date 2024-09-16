/// match keyword
/// used for pattern matching, providing a powerful control flow construct that can be used to match values
/// against patterns and run code based on which pattern is matched. 
/// It is similar to a switch statement in other languages, but much more powerful and flexible.


fn main() {

    // ----------------- Basic syntax ----------------
    // 
    // . [ _ ] pattern acts as a catch-all for any value not explicitly handled
    //  by other patterns (similar to the default case in a switch statement).

    let mut num = 3u8;
    match num {
        1 => println!("number is 1"),
        2 => println!("number is 2"),
        3 => println!("number is 3"),
        _ => println!("this is default Case like switch statement"),
    }

    // -----------------  Multpile patterns ---------


   // The code snippet `num = 15; match num { 1 | 2 | 3 => println!("number is 1 or 2 or 3"), _ =>
   // println!("this is default Case like switch statement"), }` is using the `match` keyword in Rust
   // to perform pattern matching on the value of the variable `num`.
    num = 15;
    match num {
        1 | 2 | 3 => println!("number is 1 or 2 or 3"),
        _ => println!("this is default Case like switch statement"),
    }


    // --------------- matching range ---------------
    // The code snippet `num = 8; match number { 1..=5 => println!("The number is between 1 and 5"),
    // 6..=10 => println!("The number is between 6 and 10"), _ => println!("The number is out of
    // range"), }` is using the `match` keyword in Rust to perform pattern matching on the value of the
    // variable `num`.
    num = 8;

    match num {
        1..=5 => println!("The number is between 1 and 5"),
        6..=10 => println!("The number is between 6 and 10"),
        _ => println!("The number is out of range"),
    }

    // ------------------ Matching Enums ---------

  /// This code snippet in Rust is demonstrating the use of enums and pattern matching with the `match`
  /// keyword.
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    let direction = Direction::Up;
    
    match direction {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
        _ => println!("default case"),
    }
    
}
