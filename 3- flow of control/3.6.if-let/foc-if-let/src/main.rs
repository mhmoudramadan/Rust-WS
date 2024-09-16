// let construct in Rust provides a concise way to match a pattern in situations where you care about just one specific case
// rather than needing to handle every possible pattern like you do with match
// it’s commonly used with enums or option types, especially when you only care about one variant.

//  ------------ Basic syntax ----------- 
// if let pattern = value {
//     // Code block if the pattern matches
// }

// if let is cleaner for this use case and in addition allows various failure options to be specified


fn main() {



    // The code snippet `let num = Some(7u16);` is creating a variable `num` of type `Option<u16>` and
    // assigning it the value `Some(7u16)`, which means `num` holds the value `7` of type `u16` wrapped
    // inside the `Some` variant of the `Option` enum.
    
    let num = Some(7u16);

    // The line `let letter: Option<i32> = None;` is creating a variable `letter` of type `Option<i32>`
    // and assigning it the value `None`. In Rust, `Option` is an enum that represents either Some
    // value or None, and in this case, `letter` is being assigned the value `None` which signifies
    // that there is no value present. This is a way to initialize an `Option` type with no value
    // inside it.

    let letter: Option<i32> = None;

    if let Some(i) = letter {
        println!("Match Value {:?}",i);
    }

    // For specify a failure, use an else:
    if let Some(i) = letter {
        println!("Match Value {:?}",i);
    } else {
        println!("Unmatch Value ");
    }

    
}
