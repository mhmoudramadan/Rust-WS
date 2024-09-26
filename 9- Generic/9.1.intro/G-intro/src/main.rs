// Generics

// Generics is the topic of generalizing types and functionalities to broader cases

// This is extremely useful for reducing code duplication in many ways,
// but can call for rather involved syntax. Namely, 
// being generic requires taking great care to specify over which types a generic type is actually considered valid.
// The simplest and most common use of generics is for type parameters.

// A type parameter is specified as generic by the use of angle brackets and upper camel case: <Aaa, Bbb, ...>. 
//          "Generic type parameters" 
// are typically represented as <T>. 

// In Rust, "generic" also describes anything that accepts one or more generic type parameters <T>.
//  Any type specified as a generic type parameter is generic, and everything else is concrete (non-generic).


// Basic syntax 

// fn foo<T>(arg: T) { 
        // ... 
        // }


//          ---------Remark
// Because T has been specified as a generic type parameter using <T>, it is considered generic when used here as (arg: T).
//  This is the case even if T has previously been defined as a struct.

//  Summary 
// Generic Functions: Allow functions to accept multiple types as parameters.
// Generic Structs: Allow structs to store data of any type.
// Generic Enums: Allow enums to represent values of multiple types.
// Generic Traits: Define behavior for multiple types.
// Bounded Generics: Restrict generic types to those that implement certain trait
// concrete type 
#[derive(Debug)]
struct A; 


// In defining the type `Single`, the first use of `A` is not preceded by `<A>`.
// Therefore, `Single` is a concrete type, and `A` is defined as above.
#[derive(Debug)]
struct Single(A); 


// Here, `<T>` precedes the first use of `T`, so `SingleGen` is a generic type.
// Because the type parameter `T` is generic, it could be anything, including
// the concrete type `A` defined at the top.
#[derive(Debug)]
struct SingleGen<T>(T);


fn main() {

    println!("Hello, To Generics!");

    /// The code `let _s = Single(A); println!("{:?}", _s);` is creating an instance of the `Single`
    /// struct with a concrete type `A` and then printing the debug representation of that instance.
    let _s = Single(A);
    println!("{:?}",_s);

    /// `let _char:SingleGen<char> = SingleGen('a');` is creating an instance of the `SingleGen` struct
    /// with a generic type parameter `char`. The value passed to this instance is the character `'a'`.
    /// This line demonstrates the usage of generics in Rust, where `SingleGen` can be instantiated with
    /// different types, in this case, a `char`.
    /// parameter explicitly specified
    let _char:SingleGen<char> = SingleGen('a');
    println!("{:?}",_char);


    // parameter Implicitly specified

   /// `let _t = SingleGen(A);` is creating an instance of the `SingleGen` struct with a generic type
   /// parameter. In this case, the type parameter `T` is implicitly inferred based on the value passed
   /// to the instance, which is of type `A`. This means that `T` is inferred to be the concrete type
   /// `A` in this specific instance.
    let _t    = SingleGen(A); 
   /// `let _i32 = SingleGen(6);` is creating an instance of the `SingleGen` struct with a generic type
   /// parameter. In this case, the value `6` is passed to the instance, which allows the Rust compiler
   /// to infer that the generic type parameter `T` should be of type `i32` (32-bit signed integer).
   /// This demonstrates how generics in Rust can be used with different types, in this case, an `i32`.
    let _i32  = SingleGen(6); 
    /// `let _char = SingleGen('a');` is creating an instance of the `SingleGen` struct with a generic
    /// type parameter `char`. The character `'a'` is passed as a value to this instance. This line
    /// demonstrates the usage of generics in Rust, where `SingleGen` can be instantiated with different
    /// types, in this case, a `char`. The generic type parameter `T` is explicitly specified as `char`
    /// in this instance, allowing the struct to hold a value of type `char`.
    let _char = SingleGen('a'); 


    println!("{:?}",_t);
    println!("{:?}",_i32);
    println!("{:?}",_char);


}
