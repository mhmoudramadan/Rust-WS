/// Rust program is (mostly) made up of a series of statements
/// The most common two are 
/// 1-declaring a variable binding, and using a ;
/// 2- with an expression:
/// 
/// 
/// 
/// Blocks are expressions too, so they can be used as values in assignments. 
/// The last expression in the block will be assigned to the place expression such as a local variable. 
/// if the last expression of the block ends with a semicolon, the return value will be ().
/// 


fn main() {


    let x:u64 = 100; // variable binding

    // Expression ;
    x;  // statement wit No effect
    x+5; // produce result with no effect as there is no storing variable
    15;

    println!("{}",x);  

    let y ={

        let x_2 = x*x;
        let x_3 = x_2 * x;

        x_2 + x_3 + x // expression assigned to y
    };

    let z ={
        2 * x;
        // semicolon suppresses this expression 
        // and the value assigned to z is () 
    };

    println!("{}",y);
    println!("{:?}",z);

}
