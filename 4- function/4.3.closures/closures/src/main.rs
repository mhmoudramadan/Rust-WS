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

// closures as input parameter
// When taking a closure as an input parameter, the closure's complete type must be annotated using one of a few traits,
//  and they're determined by what the closure does with captured value. In order of decreasing restriction, they are:

// Traits used

//1-     Fn:        the closure uses the captured value by reference (&T)
//2-     FnMut:     the closure uses the captured value by mutable reference (&mut T)
//3-     FnOnce:    the closure uses the captured value by value (T)

// If the parameter is annotated as Fn, then capturing variables by &mut T or T are [not allowed]. &T is allowed.

// Function that accepts a closure as a parameter

fn apply<F>(f : F) 
    where
        F: Fn(u32)->u32 ,
{
    let val = f(5);
    println!("Result is {}",val);
}


// Returning Closures from Functions

fn make_addtion(x:u32) -> impl Fn(u32) ->u32 {
    move |y| y +x
}

fn main() {

    // Basic Closure
    let sum = |a,b| a + b ;
    let mut result = sum(5,15);
    println!("Sum is {}",result);

    // Capturing Variables from the Environment

    let x:u16 = 20;
    let multi = |c| c * x;
    let result = multi(5);
    println!("Mul Result is {}",result);

    // Closures as input parameter
    let add_num = |d| d + 10;
    apply(add_num);


    let name = String::from("Mahmoud");

    // Closure that takes ownership of `name`
    let consume_name = || {
        println!("Name: {}", name);
        // `name` is consumed here, so this closure is FnOnce
    };

    // Call the closure
    consume_name();

    // `name` cannot be used anymore, as it was moved into the closure
    // println!("{}", name); // This would cause a compile-time error


    // Returning Closures from Functions
    let add_num = make_addtion(5);

    // Call closures 
    let sum_ = add_num(10);

    println!("addtion is {}",sum_);

}
