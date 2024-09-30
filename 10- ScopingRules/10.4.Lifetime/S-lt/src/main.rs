// !! LifeTime 'LT'
// * A lifetime is a construct the compiler (or more specifically, its borrow checker) uses to ensure all borrows are valid.
// * Specifically, a variable's lifetime begins when it is created and ends when it is destroyed. 
// * While lifetimes and scopes are often referred to together,
// * Lifetimes are an important concept in Rust that help ensure memory safety by preventing dangling references.

// !! LT Explicit annotation
// * The borrow checker uses explicit lifetime annotations to determine how long references should be valid
// * Rust requires explicit annotations to determine what the lifetime of a reference should be.
// * The syntax for explicitly annotating a lifetime uses an apostrophe character as follows:
// ?? syntax
// * foo<'a> -> foo` has a lifetime parameter `'a`
// ??
// * Rust needs help in understanding how references in functions relate to each other. 
// * We use lifetime annotations to specify these relationships.
// * Lifetime annotations are written with an apostrophe ('a), and they describe how long references should be valid.


// The longest function returns the longest of two string slices (&str). 
// The lifetime 'a tells the Rust compiler that the returned reference must be valid for as long as both input references are valid.
// If either s1 or s2 goes out of scope before the result, the code would fail to compile.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

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


    // ! LT Explicit annotation
    let s1 = String::from("mahmoud");
    let s2 = String::from("Ramadan");
    let result = longest(&s1,&s2);
    println!("Longest word is {}",result);


}
