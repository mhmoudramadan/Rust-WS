// struct visibility 
// The visibility defaults to private, and can be overridden with the pub modifier
// his visibility only matters when a struct is accessed from outside the module where it is defined,
//  and has the goal of hiding information (encapsulation).
// When you make a struct public using pub, its fields are still private by default
//  unless you also declare them as public.

// Key Points about Struct Visibility:

//1-A struct can be made public using the pub keyword.
//2-Even if a struct is public, its fields remain private by default.
//3-To access struct fields from outside the module, you must make both the struct
// and its fields public using pub.

mod Shapes {

    pub struct Rectangle{
        height:u32, // private field
        width:u32,  // private field
    }

    pub struct Square {
        pub length:u32, // public field
    }

    impl Rectangle {
        pub fn new (height:u32,width:u32)->Rectangle {
            Rectangle{height,width}
        }

        pub fn area(&self) ->u32 {
            self.height * self.width
        }
    }


    impl Square {
        pub fn new (length:u32) -> Square {
            Square{length}
        }

        pub fn area (&self) ->u32 {
            self.length * self.length
        }
    }
}

fn main() {

    let react_ = Shapes::Rectangle::new(10,15);

    // Error: struct fields are private
    // println!("height is {} and width is {}",react_.height ,react_.width);

    println!("area of react is {}",react_.area());


    let squ_ = Shapes::Square::new(10);

    println!("area of square is {}",squ_.area());

}
