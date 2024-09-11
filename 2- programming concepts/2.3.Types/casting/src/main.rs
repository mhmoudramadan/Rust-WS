///  No implicit conversion only explicit
///  Rust provides no implicit type conversion (coercion) between primitive types.
///  explicit type conversion (casting) can be performed using the ((as)) keyword. 
///  Rules for converting between integral types follow C conventions
/// 
/// 



fn main() {

    let float_var = 10.1045f32;

    // no implicit cast 
    // Error
    // let var:u8 =  dec;  // uncomment lead to error build

    /// explicit conversion using as keyword
    ///    
    let integer_var = float_var as u8;

    println!("before casting is {}",float_var);

    println!("after casting is {}",integer_var); 

    let char_var = integer_var as char;

    // There are limitations in conversion rules.
    // ########### Remark #####
    /// Float can not be directly convert to char 
    /// float -> integer -> char
    /// 
    println!("Casting: Float  -> Integer -> character");


    // Rules To notice 
    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // how it work
    //  1000 fits u16
    println!(" 1000 as u16 is {}", 1000 as u16);

    //  1000 - 256 -256 -256 = 232 "Fits"
    println!(" 1000 as a u8 is : {}", 1000u16 as u8);

    // the value of 232 in 8-bit two's complement representation is -24
    println!(" 232 as a i8 is : {}", 232u8 as i8);

    
}
