/**
 * Rust is a statically typed language, which means that it must know the types of all variables at compile time.
 * 
 * Every value in Rust is of a certain data type, 
 * We’ll look at two data type subsets: scalar and compound.
 * 
 *  Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
 *  Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
 *  Floating point: f32, f64
 *  char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
 *  bool either true or false
 *  The unit type (), whose only possible value is an empty tuple: ()
 * 
 * signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 
 * Unsigned variants can store numbers from 0 to 2n - 1
 * 
        * Length	Signed	Unsigned
        *  8-bit	  i8	    u8
        *   16-bit	i16	    u16
        *   32-bit	i32	    u32
        *   64-bit	i64	    u64
        *   128-bit	i128	  u128
        *   arch	  isize	   usize
      * 

 */


 /**       
  * 
    fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
    fmt::Display: Uses the {} marker. Format text in a more elegant, user friendly fashion.

Formatted print Printing is handled by a series of macros defined in std::fmt some of which are:
        1-  format!: write formatted text to String
        2-  print!: same as format! but the text is printed to the console (io::stdout).
        3-  println!: same as print! but a newline is appended.
        4-  eprint!: same as print! but the text is printed to the standard error (io::stderr).
        5-  eprintln!: same as eprint! but a newline is appended.
  *
  *
  */

 fn main(){
  
  // {} by default will be automatically replaced with any arguments. These will be stringified
    println!("{} days", 31);

 /**
  * Positional arguments can be used. Specifying an integer inside `{}`
  * determines which additional argument will be replaced 
  *Arguments start at 0 immediately after the format string. 
  */
    println!("{0}, this is {1}. {1}, this is {0}", "Mahmoud ", "Ramadan");

    // we can also named objects
    println!("{first_name} {last_name} is {age} and support {fav_club}",
    first_name ="mahmoud",
    last_name = "Ramadan",
    age ="200",
    fav_club = "El-Ahly Fc");

    // bool Type
    let flag1:bool = true;
    let flag2:bool = false;
    println!("Flag one is {flag1}");
    println!("Flag one is {flag2}");

    /**
     *  There is Two Types of annotation
     * 1 - Regular annotation
     * 2 - suffix annotation 
     * */


    // signed integer + or - values
    let num1:i8 = -100; // regular annotation 
    let num2 = 200i16; // suffix annotation 
    println!("The Num is {num1}");
    println!("Num 2 is {num2}");


      // Unsigned integer only + Values
      let num3:u8 = 100; // regular annotation 
      let num4 = 200u16; // suffix annotation 
      println!("The Num is {num1}");
      println!("Num 2 is {num2}");

      // Float 

      let fvar:f32=3.99;
      let fvar1 = 4.88f32;
      println!("The Float number is {fvar}");
      println!("Float number is {fvar1}");

      let mut mutablevar = 12;
      println!("mutable number is {mutablevar}");
    // Error 
    // mutablevar = true; // Error The type of a variable can't be changed

    // Variables can be overwritten with shadowing.
    let mutablevar = true;
    println!("mutable number is {mutablevar}");


    // Ways To create string
    /**
     * 1 - &str -> s immutable, and typically more efficient when dealing with static strings or substrings.
     * 2 - String -> s heap-allocated and mutable, useful when you need to modify or grow a string dynamically.
     */

    let name : String = String::from("hello,mahmoud");

    let name_1: &str = "Hello . mahmoud";

    println!("greeting   {}",name);
}
