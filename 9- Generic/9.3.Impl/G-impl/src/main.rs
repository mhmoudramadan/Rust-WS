// you can define generic types in structs, enums, and functions to work with multiple data types,
// as well as provide specific implementations for certain and Specific types.
// demonstrate both the use of generic types and how you can implement specific behavior for particular types using generics.

// You can implement methods for specific types by specifying the concrete type within the impl block. 
// This allows you to provide specialized behavior for different types.



// ----- Basic Demonstrate of Specific Implementation -----
// struct S; // Concrete type `S`
// struct GenericVal<T>(T); // Generic type `GenericVal`

// Specify Type
// impl of GenericVal where we explicitly specify type parameters:
// impl GenericVal<f32> {} // Specify `f32`
// impl GenericVal<S> {} // Specify `S` as defined above

// `<T>` Must precede the type to remain generic
// impl<T> GenericVal<T> {}

#[derive(Debug)]
// Generic Struct

struct Point<T> {
    x:T,
    y:T,
}

// Generic implementation for all types T

/// The line `impl<T> Point<T> {` is defining an implementation block for the generic struct `Point<T>`.
/// This means that the following functions and methods defined within this block will be applicable to
/// all instances of the `Point` struct regardless of the specific type `T` used when creating the
/// instance. This allows you to define common behavior that works for any type `T` that the `Point`
/// struct is instantiated with.
/// 
impl<T> Point<T> {

   /// The function `new` in Rust creates a new `Point` instance with the given `x` and `y` values.
   /// 
   /// Arguments:
   /// 
   /// * `x`: The parameter `x` is of type `T`.
   /// * `y`: The parameter `y` in the `new` function represents the y-coordinate of a point in a 2D
   /// space.
   /// 
   /// Returns:
   /// 
   /// A new instance of the `Point` struct with the provided `x` and `y` values is being returned.
    fn new(x:T , y:T) ->Self {
        Point{ x , y }
    }

   /// The function `x_type` returns a reference to the field `x` of the struct.
   /// 
   /// Returns:
   /// 
   /// A reference to the value of `x` in the struct.
    fn x_type(&self) -> &T {
        &self.x
    }
}

// Specific implementation for f32
/// The `impl Point<f32>` block is providing a specific implementation for the `Point` struct when it is
/// instantiated with the type `f32`. Within this block, you can define methods and functions that are
/// specific to instances of `Point` with `f32` type parameters. In this case, the `dis_f_origin` method
/// calculates the Euclidean distance of the point represented by the `Point<f32>` struct instance from
/// the origin (0,0) in a 2D space. This demonstrates how you can implement specialized behavior for
/// specific types using generics in Rust.
impl Point<f32> {

  /// The `dis_f_origin` function calculates the Euclidean distance of a point from the origin in a 2D
  /// space.
  /// 
  /// Returns:
  /// 
  /// The `dis_f_origin` function returns the Euclidean distance of the point represented by the
  /// `Point<f32>` struct instance from the origin (0,0) in a 2D space.
    fn dis_f_origin(&self) -> f32 {
        /// `(self.x.powi(2) + self.y.powi(2)).sqrt()` is calculating the Euclidean distance of the
        /// point represented by the `Point<f32>` struct instance from the origin (0,0) in a 2D space.
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn main() {

   /// The code `let int_points = Point::new(10,5); println!("integer point is {}  ,
   /// {}",int_points.x,int_points.y);` is creating a new instance of the `Point` struct with integer
   /// values `10` and `5` for the `x` and `y` coordinates respectively.
    let int_points = Point::new(10,5);
    println!("integer point is {}  , {}",int_points.x,int_points.y);

   /// The code `let float_point = Point::new(5.5,9.9); println!("float point is {}  ,
   /// {}",float_point.x,float_point.y);` is attempting to create a new instance of the `Point` struct
   /// with the values `5.5` and `9,9` for the `x` and `y` coordinates respectively.
    let float_point = Point::new(5.5,9.9);
    println!("float point is {}  , {}",float_point.x,float_point.y);

    
    /// The code `let float_point = Point::new(3.0, 4.0); println!("Distance from origin: {}",
    /// float_point.dis_f_origin());` is performing the following actions:
    let float_point = Point::new(3.0, 4.0);
    println!("Distance from origin: {}", float_point.dis_f_origin());
    
}
