// use Declaration 
// can be used to bind a full path to a new name, for easier access
// used to bring items (such as functions, structs, enums, modules, or constants) into scope
// you can use them without specifying their full paths
// this is especially helpful for reducing the verbosity of code, particularly 
// when dealing with deeply nested modules.


// Key Points about the use Declaration:

// 1- It imports modules, functions, structs, enums, or traits from other modules or crates.
// 2- It allows you to access items with their short names rather than full paths.
// 3- You can import single items or multiple items using different patterns (use module::item, use module::*, or use module::{item1, item2}).
// 4- It supports aliasing (renaming) imported items


mod Shape {
    pub fn draw_circle() {
        println!("Drawing a circle");
    }

    pub fn draw_rectangle() {
        println!("Drawing a rectangle");
    }

    pub fn draw_square(){
        println!("Drawing a square");
    }
}


// use::Shape::{draw_circle}; // Bring specific items into scope
use Shape::*;  // Bring all public items from the `shapes` module into scope

// Aliasing with as
use Shape::draw_square as draw_;

fn main() {

    Shape::draw_circle();
    Shape::draw_rectangle(); 

    draw_circle();
    draw_rectangle();

    draw_circle();
    draw_rectangle();

    draw_();
    
}
