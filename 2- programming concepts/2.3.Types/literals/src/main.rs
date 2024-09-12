/// Numeric literals can be type annotated by adding the type as a suffix
/// the type of unsuffixed numeric literals will depend on how they are used
/// If no constraint exists, the compiler will use i32 for integers, and f64 for floating-point numbers
///  



fn main() {

    // suffixed literals
    let x =10u8;
    let y = 300u16;
    let z = -10i8;
    let p = 3f32;



    println!("------------Suffixed literals----------");

    println!("size of x is {}   Byte",std::mem::size_of_val(&x));
    println!("size of y is {}   Byte",std::mem::size_of_val(&y));
    println!("size of z is {}   Byte",std::mem::size_of_val(&z));
    println!("size of p is {}   Byte",std::mem::size_of_val(&p));



    // unsuffixed literals

    let p = 15;
    let f = 1.0;

    println!("-------Unsuffixed literals!---------");
    println!("size of p is {}   Byte",std::mem::size_of_val(&p));
    println!("size of f is {}   Byte",std::mem::size_of_val(&f));
 
  
}
