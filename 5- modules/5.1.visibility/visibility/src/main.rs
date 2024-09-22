// modules allow you to organize and manage code in a hierarchical and structured way. 
// Modules can contain functions, structs, enums, constants, and even other modules.
//  They help create clean and maintainable code by controlling the visibility (public/private) of items within a module.
// A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.


// Key Points about Modules:

//     Items in modules default to private visibility.
//     Modules group related code.
//     Items within a module can be made public or private using the pub keyword.
//     Nested modules allow for a hierarchical structure.
//     Modules can be defined in the same file or split across multiple files.

//              Defining a Module
// You define a module using the mod keyword. By default, 
// everything inside a module is private unless explicitly made public using the pub keyword.

mod my_default_modules {

    fn private_fun() {
        println!("this is private fun and it's private visibility");
    }

    pub fn public_fun() {
        println!("public visibility function inside module");
    }

    // nested modules
    pub mod my_nested_module {
        pub fn nested_module (){
            println!("Nested Module function");
        }
    }
}

fn main() {

        //  cause an error because `add` is private   
//  my_default_modules::private_fun();
    my_default_modules::public_fun();
    my_default_modules::my_nested_module::nested_module();

}
