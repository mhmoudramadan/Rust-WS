// Multiple bounds for a single type can be applied with a [ +  ]. 
// Like normal, 
// different types are separated with [ , ].

use std::fmt::{Debug , Display};

/// The function `com_prints` in Rust takes a generic type that implements both `Display` and `Debug`,
/// and prints its value using both formatting traits.
/// 
/// Arguments:
/// 
/// * `valu`: The parameter `valu` is a reference to a value of type `T`, where `T` implements both the
/// `Display` and `Debug` traits. The function `com_prints` takes this reference as an argument and
/// prints out the value in both debug format using `{:?}` and
fn com_prints<T : Display + Debug > (valu :&T){
    println!("Debug : {:?}",valu);
    println!("Display : {}",valu);
}

/// The function `com_types` in Rust prints the debug representation of two values of different types.
/// 
/// Arguments:
/// 
/// * `val1`: The function `com_types` takes two generic parameters `T` and `U` that must implement the
/// `Debug` trait. The function then takes two references `val1` and `val2` of types `T` and `U`
/// respectively, and prints out their debug representations using `
/// * `val2`: The function `com_types` takes two generic parameters `T` and `U`, where both `T` and `U`
/// must implement the `Debug` trait. The function then takes two references `val1` and `val2` of types
/// `T` and `U` respectively.
fn com_types<T : Debug, U: Debug> (val1 : &T , val2 : &U) {

    println!("val1 is {:?}",val1);
    println!("val2 is {:?}",val2);
}

fn main() {
    let my_str = String::from("Mahmoud");
    let my_arr = [10,20,30];
    let my_vec = vec![100,20,50];
    
    com_prints(&my_str);
    com_types(&my_arr,&my_vec);

}
