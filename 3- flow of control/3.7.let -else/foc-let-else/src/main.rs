// It allows you to match a pattern and provide a fallback or "else" block if the pattern fails.
// This is particularly useful when handling types like Option or Result, 
// where you want to immediately return or handle an error when a pattern doesn't match.

// Syntax 
// let pattern = expression else {
//     // Fallback code if the pattern does not match
// };

// if the pattern does not match, the code inside the else block is executed.
// This often includes things like returning an error, panicking, or handling an alternative case.

fn main() {
    
    let some_value : Option<i32> = None;

    // Handling Option with let-else
     // Attempt to extract value from Option
    let Some(x) = some_value else {
        println!("No value found");
        return;
    };

}
