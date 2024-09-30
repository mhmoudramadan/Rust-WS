// !! LifeTime 'LT'
// * A lifetime is a construct the compiler (or more specifically, its borrow checker) uses to ensure all borrows are valid.
// * Specifically, a variable's lifetime begins when it is created and ends when it is destroyed. 
// * While lifetimes and scopes are often referred to together,
// * Lifetimes are an important concept in Rust that help ensure memory safety by preventing dangling references.



fn main() {
    println!("Hello, Life time concepts!");
    // ! Basic LT concept
    // ? The borrow has a lifetime that is determined by where it is declared. 
    // ? As a result, the borrow is valid as long as it ends before the lender is destroyed.

    let x:u32 =10; // Lifetime for `x` starts.

    {
        let b1 = &x; // b1 life time 
        println!("Borrow 1 is {}",b1);
    } // b1 life time ends

    {
    let b2 = &x; // b2 lifetime starts
    println!("Borrow 2 is {}",b2);
    } // b2 life time ends
}
