//  Variable bindings have a scope, and are constrained to live in a block. 
//  block is a collection of statements enclosed by braces {}.

/// #variable 

fn main() {
    // variable binding lives in the main function
    let long_bind_no:u32 = 100; 

    // small block {
    {
        println!("{}",long_bind_no); 
        
        let short_bind_no:u32 = 200;
        println!("{}",short_bind_no);  
    }
    println!("{}",long_bind_no);
    // println!("{}",short_bind_no);  // Erorr outof scope


    // Variable shadowing
    // occurs when a new variable with the same name as a previous one is declared in the same scope or an inner scope.
    // The new variable "shadows" the old one, meaning the previous value becomes inaccessible
    // and the new variable takes its place. 
    // Shadowing allows to reuse variable names while preserving immutability


    let x_shadowing :u32 = 10; // first declaration
    println!("first declaration is {}",x_shadowing);  // output is ->  10


    let x_shadowing = x_shadowing +1 ; // shadowing  x_shadowing with new value 
    println!("first shadowing is {}",x_shadowing); // output is -> 11


    {
        // Inner scope
        // shadowing  x_shadowing in inner scope and the new alue lives in inner scope life
        let x_shadowing = x_shadowing * 2 ; 
        println!("inner scope shadowing is {}",x_shadowing); // output is -> 22
    }

    //  x_shadowing remain as same last value in outer scope
    println!("variable remain value as last outer scope shadowing {}",x_shadowing); // output is -> 11
}
