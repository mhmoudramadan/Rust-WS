/// for in  range construct can be used to iterate through an Iterator
/// One of the easiest ways to create an iterator is to use the range notation a..b
/// This yields values from a (inclusive) 
///                    to   b (exclusive) in steps of one.
/// 
///  



fn main() {

    // for and range
    /// This code snippet is using a `for` loop with a range to iterate from 1 (inclusive) to 100
    /// (exclusive).
    for n in 1..100 {
        if n % 10 == 0 {
            println!("Valid division Number is {}",n);
        } else {
            println!("Invalid division is {}",n)
        }
    }

  /// The code snippet `for n in 1..=100 { println!("N is {}", n); }` is using a `for` loop with a range
  /// to iterate from 1 (inclusive) to 100 (inclusive). This means that the loop will iterate over
  /// values starting from 1 up to and including 100. Inside the loop, it will print out the value of
  /// `n` with the message "N is [value]".
    for n in 1..=100 {
        println!("N is {}",n);
    }
}
