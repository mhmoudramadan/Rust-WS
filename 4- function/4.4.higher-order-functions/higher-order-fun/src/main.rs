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


// Returning a Closure from a Function
// HOF

/// The function `create_mul` takes a `multiplier` as input and returns a closure that multiplies its
/// input by the `multiplier`.
/// 
/// Arguments:
/// 
/// * `multiplier`: The `multiplier` parameter is of type `u32`, which stands for an unsigned 32-bit
/// integer. It is used to create a closure that multiplies its input by this `multiplier` value.

fn create_mul(multiplier:u32) -> impl Fn(u32)->u32{
    // move keyword captures multiplier by value, allowing it to be used inside the closure.
    move |x| x*multiplier
}

// HOF with Iterators
// using iterator

fn is_odd(n:u32)->bool {
    n %2 == 1
}

fn main() {
    // Pass `add` function to `operation`
    let mut result = operation(add, 30, 10); 
    println!("Result: {}", result);
   // |x| x * 2 is passed to apply, which doubles the value of 5.
    result = apply(|x|x*2,5);
    println!("Result: {}", result);

     // Create a closure that multiplies by 3
    let mul_by_3 = create_mul(3);

     // Call the closure
    result = mul_by_3(10);
    println!("Result: {}", result);

    //  find sum of all squared odd under 1000
    println!("Find the sum of all the numbers with odd squares under 1000");
    let upper_limit :u32 = 1000;

    let mut counter = 0u32;

    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        let n_squared = n*n;

        if n_squared > upper_limit {
            // break if it reach limit
            break;
        }else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            counter +=n_squared;
        }
        else {
            println!("Invalid statment");
        }
    }
    
    println!("accumalter counter is : {}", counter);

    // functional approach using some iterators
    let sum_of_squared_odd :u32 =
        (0..).map(|n| n*n)  // All natural numbers squared
        .take_while(|&n_squared| n_squared < upper_limit) // check limit
        .filter(|&n_squared| is_odd(n_squared))     // That are odd
        .sum(); // sum

    println!("sum of squared odd is {}",sum_of_squared_odd);

}
