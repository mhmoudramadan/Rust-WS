


/// inference engine is smart
/// It does more than looking at the type of the value expression during an initialization.
/// It also looks at how the variable is used afterwards to infer its type.
///  


fn main() {

    let x = 10u16; 

    let mut vec = Vec::new(); // unknown Vector type

    // Inference Engine vec` is a vector of u16 
    vec.push(x); 

    println!("{:?}",vec);
    
}
