// In Rust, a crate is the basic unit of compilation and the package system. 
// A crate can be either a binary or a library. Crates are defined with a Cargo.toml file,
//  which manages the crate’s dependencies, metadata, and build instructions. 



// Key Concepts of Crates in Rust

//     Crates: The smallest compilation unit in Rust, which can be a binary (executable) or a library (reusable code).
//     Cargo: Rust's build system and package manager, used to create, manage, and build crates.
//     Dependencies: Crates can depend on other crates, which are fetched from the crates.io repository.
//     Crate Types:
//         Binary Crates: Contain an executable main.rs.
//         Library Crates: Contain reusable code (lib.rs) that other crates can use.


// 1 - create a binary crate 'main.rs'
fn main() {
    
    println!("This is a binary crate!");
}
