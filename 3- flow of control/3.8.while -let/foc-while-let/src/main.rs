//  loop construct that repeatedly runs a block of code while a pattern matches
// It's particularly useful when you want to loop until an Option or Result becomes None or Err.
// Instead of using a combination of loop and match, while let offers a more concise way to handle looping with pattern matching.
//  -------------- syntax
// while let pattern = expression {
//     // Code block that runs as long as the pattern matches
// }


fn main() {


    //  Looping with Option
    let mut value = Some(3);

    while let Some(x) = value {
        println!("Value is {}",x);

        value = if x > 0 {
            Some(x-1)
        } else {
            None
        };
    }

    println!("No more values!");

    // using vector 

    let mut stack = vec![1,2,3];

    while let Some(top) = stack.pop() {
        println!("Popped value: {}", top);
    }

    println!("Stack is Empty");

    // Handling Result

    let mut attempts = vec![Ok(1), Ok(2), Err("Error"), Ok(3)];

    while let Some(Ok(value)) = attempts.pop() {
        println!("Processed value: {}", value);
    }

    println!("Stopped processing due to error or no more attempts.");

}
