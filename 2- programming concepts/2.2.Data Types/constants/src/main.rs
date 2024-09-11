// constants are immutable values that are bound to a name and cannot be changed after they are declared.
// constants are evaluated at compile-time and must have a type explicitly annotation


//                   Characteristics of Constants:

/**
 * 
 *   
  1-Global Scope: Constants can be defined at any scope, 
                 including global scope, and they can be accessed anywhere within their scope.

 2-Compile-Time Evaluation: Constants are evaluated at compile-time, so their values must be known at that point

 3-Cannot Be Shadowed: Unlike variables, constants cannot be shadowed (redefined in the same scope).

 4-Static Lifetime: Constants have a 'static lifetime, which means they are valid for the entire duration of the program.


 */
// Types of declaraing constants 
/**
 *  1 - const :An immutable "unchangeable" value (the common case).
 * 
 * 
 *  2 - static :A possibly mutable variable with 'static lifetime. 
 *  The static lifetime is inferred and does not have to be specified. 
 *  Accessing or modifying a mutable static variable is ----->(unsafe).
 */


//  Globals Scope for constants 
    static NAME:& str = "Mahmoud";
    const PI:f32 = 3.14;
    const MAX_SIZE: u32 = 100;


 fn main(){

    println!("{}",NAME);
    println!("{}",PI);
    println!("{}",MAX_SIZE);
 }