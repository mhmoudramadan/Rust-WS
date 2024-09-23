

// the super and self keywords are used within modules to navigate between different levels of the module hierarchy. 
// They help you refer to items within the current module or in the parent module, allowing you to organize code more effectively.


// Key Points:

//     self: Refers to the current module or scope.
//     super: Refers to the parent module, one level up in the module hierarchy.

// These keywords help in resolving paths to access items like functions, structs, or constants defined in the same module or in the parent module.


mod Math {
    pub mod Geometery {

        pub fn area_of_square(length:u32) ->u32 {
            length * length
        }

        pub fn perimete_of_square(length:u32)->u32 {
            length * 4
        }

        pub fn calculate_all_stuff(length:u32) {
            let area_ = self::area_of_square(length);
            let perimeter_ = self::perimete_of_square(length);

            println!("Area is {} and perimeter is {}",area_,perimeter_);
        }

    }
}

fn main() {

    Math::Geometery::calculate_all_stuff(20);
}

