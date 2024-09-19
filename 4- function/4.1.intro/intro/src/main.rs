// Rust code uses snake case as the conventional style for function and variable names, 
// in which all letters are lowercase and underscores separate words.

// Fun defnintion
// We define a function in Rust by entering fn followed by a function name and a set of parentheses.
// The curly brackets tell the compiler where the function body begins and ends.


// fun Calling
// We can call any function we’ve defined by entering its name followed by a set of parentheses.

// Remark 
// Rust language dosern't care about prescdence in create fun 
// that mean fun create define before or after its call doesn't care about it 
// it only care about defining somewhere in a scope that can be seen by the caller


fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
