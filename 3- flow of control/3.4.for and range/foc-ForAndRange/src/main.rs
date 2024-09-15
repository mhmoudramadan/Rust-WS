/// for in  range construct can be used to iterate through an Iterator
/// One of the easiest ways to create an iterator is to use the range notation a..b
/// This yields values from a (inclusive) 
///                    to   b (exclusive) in steps of one.
/// 
///  



fn main() {

    // for and range
    /// This code snippet is using a `for` loop with a range to iterate from 1 (inclusive) to 100
    /// (exclusive).
    for n in 1..100 {
        if n % 10 == 0 {
            println!("Valid division Number is {}",n);
        } else {
            println!("Invalid division is {}",n)
        }
    }

  /// The code snippet `for n in 1..=100 { println!("N is {}", n); }` is using a `for` loop with a range
  /// to iterate from 1 (inclusive) to 100 (inclusive). This means that the loop will iterate over
  /// values starting from 1 up to and including 100. Inside the loop, it will print out the value of
  /// `n` with the message "N is [value]".
    for n in 1..=100 {
        println!("N is {}",n);
    }

    // for and iterators 
    // for in range construct is able to interact with an Iterator in several ways
    // for use iterator trait like -> iter ,into_iter and iter_mut 
    // all handle the conversion of a collection into an iterator in different ways
    // by providing different views on the data within.

    // ### 1-  iter 
    // borrows each element of the collection through each iteration
    // collection untouched and available for reuse after the loop

/// This code snippet is creating an array named `names` containing three string references. It then
/// iterates over each element of the `names` array using the `iter()` method, which returns an iterator
/// over the elements of the array.
/// 
    let names:[&str ; 3]  = ["mahmoud","ahmed","ali"];

    for name in names.iter() {

        match name {   // Refernce so need to add & before str 
            // or 
        // match *name{ . // Dereference the reference  so no need to &
        // "ahmed" => println!("there is no name like this  "),
        
            &"ahmed" => println!("Found Matching "),

            _ => println!("Hello , {}",name),

        }
    }
    println!("names: {:?}", names);


    // ### 2- into_iter
    // This consumes the collection so that on each iteration the exact data is provided. 
    // Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.


    /// This code snippet is creating a vector named `ccl` containing three string elements. It then
    /// iterates over each element of the `ccl` vector using the `into_iter()` method, which consumes the
    /// vector on each iteration providing the exact data. Inside the loop, it matches each element against
    /// the string "ali" and prints "Found matching!" if there is a match, otherwise it prints "Hello
    /// [element]".
    let ccl = vec!["mahmoud", "ahmed", "ali"];

    for x in ccl.into_iter() {
        match x {
            "ali" => println!("Found matching!"),
                        _ => println!("Hello {}",x),
        }
    }
        
    // println!("names: {:?}", ccl);
    // ### 3- iter_mut
    // This mutably borrows each element of the collection
    //  allowing for the collection to be modified in place.

    let mut names = vec!["mahmoud", "ahmed", "ali","mahmoud"];

    for name in names.iter_mut() {
        *name = match name {

            &mut "mahmoud" => "Found matching and modify it ",
            _ => name,
        }
    }

    println!("names: {:?}", names);



}
