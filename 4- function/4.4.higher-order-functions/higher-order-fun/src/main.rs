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
fn operation<F>(f:F,z:u32,c:u32) -> u32
    where
    F:Fn(u32,u32)->u32, 
    {

        // Call the passed function
        f(z,c)
    }
    


fn main() {
    // Pass `add` function to `operation`
    let result = operation(add, 30, 10); 
    println!("Result: {}", result);
    
}
