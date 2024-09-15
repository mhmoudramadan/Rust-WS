/// while keyword can be used to run a loop while a condition is true.
/// Repeats as long as a condition is true.
/// typically used when the number of iterations is not known beforehand
///  

fn main() {
   /// The code snippet is using a `while` loop in Rust to decrement the value of the variable `num`
   /// from 15 to -2.
    let mut num = 15i8;

    while num != -2 {
        println!("Number is {}", num);
        num -=1;
    }
}
