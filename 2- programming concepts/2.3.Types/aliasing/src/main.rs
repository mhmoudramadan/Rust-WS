/// .  Aliasing
///  giving an new name to existing type like "typedef in C-Language"
///  to make an alias you should use " type " keyword
///  --- how to use
///  [type-keyword] [New name] = [type you want to make alias of it]
///  
///  



fn main() {


    type uint8 = u8;
    type int8 = i8;

    type uint16 = u16;
    type int16  = i16;

    type uint32 = u32;
    type int32 = i32;

    type uint64 = u64;
    type int64 = i64;

    type uint128 = u128;
    type int128 = i128;


    let x:uint8 = 100 ;

    println!("the size of uint8 is {} byte ", std::mem::size_of_val(&x));
    println!("x = {}" , x);

    
}
