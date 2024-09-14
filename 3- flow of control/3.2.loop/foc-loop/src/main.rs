/// Loop keyword used to define infinte loop 
/// break statement can be used to exit a loop at anytime
/// continue statement can be used to skip the rest of the iteration and start a new one.
/// 
/// 
/// 
/// Nesting and Labels
/// It's possible to break or continue outer loops when dealing with nested loops
/// the loops must be annotated with some 'label like 'outer and inner
/// and the label must be passed to the break/continue statement.
/// 
/// 


fn main() {
    let mut counter = 15u8;

    println!("Let's count until break!");

   /// The code snippet you provided is a Rust program that demonstrates the use of a loop with
   /// conditional statements. Here's a breakdown of what the code does:
    loop {
        counter +=1;


        if counter == 18 {
            println!("continue statement at 18 ");
            continue;  // Skip the rest of this iteration
        }

        println!("{}", counter);

        if counter == 20 {
            println!(" break statement");

             // Exit this loop
            break;
        }
    }


    // Returning from loop
/// The code snippet `let result = loop { counter += 1; if counter == 30 { break counter * 2; } };` is
/// creating a loop that increments the `counter` variable by 1 in each iteration. When the `counter`
/// reaches 30, the loop breaks and the value of `counter * 2` is assigned to the variable `result`.
    let result = loop {
        counter += 1;

        if counter == 30 {
            break counter * 2;
        }
    }; 


    // Nesting loops and Labels
    //  label start with --> 'label: loop{}
    /// The code snippet you provided demonstrates the concept of nesting loops and using labels in
    /// Rust. Here's a breakdown of what the code does:
    'outer: loop {

        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            // break; 

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached because we break the outer loop from inner loop"); 
    }

    println!("Exited the outer loop");
}
