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


// !! LT with function 
// * Function signatures with lifetimes have a few constraints:
// * any reference must have an annotated lifetime.
// *  any reference being returned must have the same lifetime as an input or be static.
    // ? remark
// * that returning references without input is banned if it would result in returning references to invalid data

// One input reference with lifetime `'a` which must live
// at least as long as the function.
fn print_one<'a>(x: &'a i32) {
    println!("x is {}", x);
}

// Mutable references are possible with lifetimes as well.
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifetimes. In this case, it
// would be fine for both to have the same lifetime `'a`, but
// in more complex cases, different lifetimes may be required.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {}, y is {}", x, y);
}

// Returning references that have been passed in is acceptable.
// However, the correct lifetime must be returned.
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
     x 
    }

// !! LT in methods
// * When defining methods on structs with references, 
// * lifetime annotations may be required to ensure that the method and its parameters follow Rust’s borrowing rules.

struct Counter(u32);

impl Counter {

    // This method increments the value in the struct by 1
    fn increment<'a>(&'a mut self) {
        self.0+=1;
    }
}

// !! LT with structs
// * Lifetimes can also be used with structs when the struct contains references. 
// * The lifetime ensures that the data being referenced is valid for as long as the struct exists

#[derive(Debug)]
struct Borrowing_Named<'a> {
    x : &'a u32,
    y : &'a u32,
}


// !! LT with Coercion
// * Lifetime coercion in Rust refers to the automatic conversion of lifetimes in certain contexts, 
// * where Rust allows shorter lifetimes to be extended to match longer ones. 
// * This is done to make it easier to work with function calls and references without requiring explicit lifetime annotations every time.

fn print_longest<'a>(x: &'a str, y: &'a str) {
    println!("Longest string: {}", if x.len() > y.len() { x } else { y });
}


// !! LT with Static
// * the 'static lifetime is a special lifetime that represents references that live for the entire duration of the program.
// * Global variables in Rust are marked with the static keyword, and they always have a 'static lifetime.

// ? 'static Lifetime with Non-Literal Data
static GL_MSG :&str = "Global ST - lT";

// ?? Static References in Data Structures
// * You can use 'static lifetimes in structs when you want a field to hold a reference that lives for the entire program.
// * This is useful for cases where the data is either constant or will never be deallocated.

struct Config<'a>{
    msg:&'a str,
}


// !! LT wit elision
// * Some lifetime patterns are overwhelmingly common and 
// * so the borrow checker will allow you to omit them to save typing and to improve readability.
// ? elision
// `elided_input` and `annotated_input` essentially have identical signatures
// because the lifetime of `elided_input` is inferred by the compiler:
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

// Similarly, `elided_pass` and `annotated_pass` have identical signatures
// because the lifetime is added implicitly to `elided_pass`:
fn elided_pass(x: &i32) -> &i32 { x }

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

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


    // ! LT with Function paramater

    let y = 5;
    let z =100;
    let mut d = 6;
    print_one(&y);
    print_multi(&y,&z);
    let s = pass_x(&z,&y);
    print_one(&s);
    add_one(&mut d);
    print_one(&d);

    // ! LT in methods
    let mut counter = Counter(10);
    counter.increment();
    println!("counter is {}",counter.0);

    // ! LT with structs
    let (b6,b7) =(10,15);
    let borrow_ = Borrowing_Named{x :&b6 ,y:&b7};
    println!("LT with struct is {:?}",borrow_);


    // ! LT with Coercion
    let string1 = String::from("hello");
    let string2 = "world";

    // string2 has a 'static lifetime (it's a string literal)
    // string1's lifetime is shorter, but Rust automatically coerces it
    print_longest(&string1, string2); // Coercion happens here


    // !! LT with Static
    // ? String Literals with 'static Lifetime
    let s_lt : &'static str = "string literals LT"; //  `'static` lifetime string
    println!("s-LT is{}",s_lt);
    // ? 'static Lifetime with Non-Literal Data
    println!("{}",GL_MSG);
    // ? Static References in Data Structures
    let config:Config<'static> = Config{
        msg: "This message will live for the entire program",
    };
    println!("Config message: {}", config.msg);


    // ! LT with elision
    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));

}
