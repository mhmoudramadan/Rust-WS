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

    // This code snippet in Rust is demonstrating the use of enums and pattern matching with the `match`
    // keyword.
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


    // Destructuring using match
    // you can destructure complex types like structs, tuples, and enums with associated data.
    
    // This code snippet in Rust is demonstrating pattern matching with destructuring on a tuple.
    let point = (3, 4);

    match point {
        (0, 0) => println!("The point is at the origin"),
        (x, 0) => println!("The point is on the x-axis at {}", x),
        (0, y) => println!("The point is on the y-axis at {}", y),
        (x, y) => println!("The point is at ({}, {})", x, y),
        _ => println!("Invalid point coordinates"),
    }


    // pointers and ref
    // a distinction needs to be made between destructuring and dereferencing 
    // as they are different concepts which are used differently from languages like C/C++.


    // Dereferencing uses ------->   *
    // Destructuring uses-------->  & , ref , and  ref mut

   // `let reference = &4;` is creating a variable named `reference` that holds a reference to the
   // value `4`. The `&` symbol is used to create a reference to the value `4` in Rust. This means that
   // `reference` is not directly storing the value `4`, but rather a reference to where the value `4`
   // is stored in memory. This allows you to work with the value `4` indirectly through the reference
   // `reference`.
   // 
   // 
    
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }


    // ------------ Guards -----------
    // used for filter the match

    enum Point {
        x(i16),
        y(i16),
    }

    let point = Point::x(35);

    match point {
        // The `if condition` part ^ is a guard
        Point::x(T) if T > 30 => println!("point located at x axis point after 30 "),

        Point::x(T) => println!("point located at x axis point after 30 "),

        Point::y(T) if T > 30 => println!("point located at x axis point after 30 "),

        Point::y(T) if T > 30 => println!("point located at x axis point after 30 "),
        _ => println!("default case"),
    }
    
}
