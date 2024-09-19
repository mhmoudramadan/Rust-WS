// closures are anonymous functions that can capture variables from their surrounding environment. 
// They are flexible and can be used where functions are used but with more capabilities. 
// Closures are commonly passed as arguments to functions and can store data


// The syntax and capabilities of closures make them very convenient for on the fly usage. 
// Calling a closure is exactly like calling a function. However, 
// both input and return types can be inferred and input variable names must be specified.

// Other characteristics of closures include:

//     using || instead of () around input variables.
//     optional body delimitation ({}) for a single line expression (mandatory otherwise).
//     the ability to capture the outer environment variables.


// Key Features of Closures:

//     Anonymous: Closures don't have names like functions.
//     Capture Variables: Closures can capture variables from their environment.
//     Flexible: They automatically infer argument types and return types (though types can also be specified).
//     Fn Traits: Closures implement one of the Fn, FnMut, or FnOnce traits, depending on how they capture variables.

//  Syntx 
// let closure = |parameter1, parameter2| {
//     // Body of the closure
//     // Can capture variables from the surrounding scope
// };



fn main() {

    // Basic Closure
    let sum = |a,b| a + b ;
    let result = sum(5,15);
    println!("Sum is {}",result);
}
