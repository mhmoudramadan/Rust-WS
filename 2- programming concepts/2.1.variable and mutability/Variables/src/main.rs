// By default, variables in Rust are immutable.
// so that mean once we give a variable a value --> the value cannot won't change
// when we want to change value we must add {mut} keyword before variable

// let x = 10; immutable "Value won't change"
// let mut x = 100;  mutable "value can be change"

fn main (){

    let x = 100;
    println!("the values of immutable x is {x}");

    // Error  -> reassignment of immutable varible
    //x = 200;
    //println!("the values of x is {x}");  


    let mut y = 300;
    println!("The Value of Mutable Y is {y}");

    y = 1000;
    println!("The Value of Mutable Y is {y}")
}