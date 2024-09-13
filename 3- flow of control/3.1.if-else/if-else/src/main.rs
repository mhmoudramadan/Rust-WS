/// if / else -> conditional expression allows you to execute different code branches based on conditions.
/// Imp Remark
/// if/else in Rust is an expression, meaning it can return a value.
///  types of if/else
///     1- Basic if/else Syntax
///     2- nesting if
///     3- Using if as an Expression 
///     4- Condition Must Return the Same Type
///     5- Nesting if/else
/// 
 

fn main() {

   // 1- Basic if/else Syntax 
    let mut x : u8 = 100;

    if x > 10 {
        println!("Number is bigger than 10 and it's {}",x);
    } else {
        println!("Number is bigger than 10 and it's {}",x);
    }

    // 2- nesting if

    if x < 10 {
        println!("Number is less than 10 it's {}",x);
    } else if x>30 && x<60{
        println!("Number is between 30 and 60  and it's {}",x);
    }else if x >60 && x<99 {
        println!("Number is between 60 and 99 and it's {}",x);        
    } else {
        println!("Number is bigger than 99 and it's {}",x);
    }

    //  3- Using if as an Expression 

    let cond:bool = true;

    let number = if cond {100} else {50} ;

    println!("The value of number is {}",number);

    x = 10;

    let y = 
    if x > 5 {
        println!("y is bigger than x");
        x * 10 // no semicolon to make it as expression and return result to assigned in y
    } else {
        println!(" y is small than x ");

        x*5  // no semicolon to make it as expression and return result to assigned in y

    }; // semicolon ;  here this because "let"

    println!("Y is {} and x is {}",y,x);


    // 4- Condition Must Return the Same Type
    // both branches must return the same type

    // This will cause a compile-time error
    //  let number_1 = if condition { 5 } else { "ten" }; // compile Error must be same type 

    //  println!("The value of number is: {}", number_1); 

    // 5- Nesting if/else

    
    let num = 15;

    if num % 2 == 0 {
        if num < 10 {
            println!("Even number less than 10");
        } else {
            println!("Even number greater than or equal to 10");
        }
    } else {
        if num < 10 {
            println!("Odd number less than 10");
        } else {
            println!("Odd number greater than or equal to 10");
        }
    }
    
}
