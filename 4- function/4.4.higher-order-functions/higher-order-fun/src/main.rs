//  HOF
// functions that either take one or more functions as arguments or return a function
// Key Points:
//  Higher-order functions take other functions or closures as parameters or return them.
//  You can pass functions or closures as arguments to functions, and return them as results from functions.


// Passing a Function as an Argument

// Simple fun
fn add(x:u32,y:u32) ->u32 {
    x+y
}

// HOF
/// The function `operation` takes a closure `f` that accepts two `u32` parameters and returns a `u32`,
/// along with two `u32` parameters `z` and `c`, and calls the closure with `z` and `c` as arguments.
/// 
/// Arguments:
/// 
/// * `f`: The parameter `f` in the `operation` function is a closure that takes two `u32` parameters
/// and returns a `u32` value. This closure is passed as an argument to the `operation` function and is
/// then called within the function body with the values `z` and `
/// * `z`: `z` is a parameter of type `u32`, which is an unsigned 32-bit integer.
/// * `c`: The parameter `c` is of type `u32`, which stands for an unsigned 32-bit integer in Rust. It
/// is one of the input parameters for the `operation` function.
/// 
/// Returns:
/// 
/// The function `operation` is returning the result of calling the function `f` with the arguments `z`
/// and `c`.
fn operation<F>(f:F,z:u32,c:u32) -> u32
    where
    F:Fn(u32,u32)->u32, 
    {

        // Call the passed function
        f(z,c)
    }
    
//  closure as a parameter
// HOF
/// The `apply` function in Rust takes a closure `f` that operates on a `u32` value and applies it to
/// the given `value`, returning the result.
/// 
/// Arguments:
/// 
/// * `f`: The parameter `f` in the `apply` function is a closure or function that takes a `u32` as
/// input and returns a `u32` as output.
/// * `value`: The `value` parameter is a 32-bit unsigned integer that is passed as an argument to the
/// function `f`.
/// 
/// Returns:
/// 
/// The `apply` function takes a closure `f` that accepts a `u32` parameter and returns a `u32` value.
/// It then applies this closure to the `value` parameter and returns the result.

fn apply<F>(f: F, value: u32) -> u32
where
    F: Fn(u32) -> u32,
{
    f(value)
}

fn main() {
    // Pass `add` function to `operation`
    let mut result = operation(add, 30, 10); 
    println!("Result: {}", result);
   // |x| x * 2 is passed to apply, which doubles the value of 5.
    result = apply(|x|x*2,5);
    println!("Result: {}", result);
    
}
