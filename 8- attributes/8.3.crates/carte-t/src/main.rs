// The crate_type attribute can be used to tell the compiler whether a crate is a binary or a library (and even which type of library)
// the crate_name attribute can be used to set the name of the crate.

// Remark
// it is important to note that both the crate_type and crate_name attributes have no effect whatsoever when using [ Cargo ]
// they are used with [rustc]

// This crate is a library
#![crate_type = "lib"]
// The library is named "rary"
#![crate_name = "mahmoud"]

pub fn public_function() {
    println!("called mahmoud's `public_function()`");
}

fn private_function() {
    println!("called mahmoud's `private_function()`");
}

pub fn indirect_access() {
    print!("called mahmoud's `indirect_access()`, that\n> ");

    private_function();
}


// How To build lib
// cd to file.rs
// command --> rustc file.rs
