// Configuration conditional checks are possible through two different operators:

//     the cfg attribute: #[cfg(...)] in attribute position
//     the cfg! macro: cfg!(...) in boolean expressions

// While the former enables conditional compilation, the latter conditionally evaluates to true or false literals 
// allowing for checks at run-time. Both utilize identical argument syntax.


// cfg!, unlike #[cfg], 
// does not remove any code and only evaluates to true or false.
//  For example, 
// all blocks in an if/else expression need to be valid when cfg! is used for the condition,
// regardless of what cfg! is evaluating

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_u_linux(){
    println!(" you are a linux system");
}

// This function only gets compiled if the target OS is Not linux
#[cfg(not(target_os = "linux"))]
fn are_u_linux(){
    println!(" you are Not linux system");
}


fn main() {
    are_u_linux();

    if cfg!(target_os = "linux"){
        println!(" It's linux OS!");
    } else {
        println!(" It's *not* linux OS!");
    }
}
