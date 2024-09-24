
// --- File Hierarchy -----
//  TODO: Explain

// you can structure your project into multiple files and modules to better organize your code, 
// especially for larger projects. The file hierarchy in Rust is designed around the module system, 
// where each file represents a module, and directories can represent submodules.


// Key Concepts 

    // 1- main.rs or lib.rs: These are the entry points for binary or library projects.
    //    - main.rs is used for binary applications.
    //    - lib.rs is used for libraries.
    // 
    // 2- Modules: Each file can represent a module. Nested modules can be created by creating subdirectories and files.
    // 3- File structure mirrors the module structure: Directories are used for submodules, and files represent individual modules.


        //  ? Example of file hierarchy
//     my_project/ 
// ├── src/
// │   ├── main.rs
// │   ├── mod1.rs
// │   ├── mod2/
// │   │   ├── mod2.rs
// │   │   └── submod.rs
// │   └── mod3.rs
// ├── Cargo.toml

mod mod1;
mod mod2;
mod mod3;
 
fn main() {
    println!("Entry point of files");
    mod1::greeting();
    mod2::say_hello();
    mod2::submod::say_goodbye();
    mod3::introduce();

}
