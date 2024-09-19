// Associated functions & Methods

// -> are functions defined within an impl block that are associated with a specific type, such as a struct, enum, or trait

// Key diff
// associated function does not take [ self ] as parameter
//  methods Take [self] as parameter

// Some functions are connected to a particular type like 'struct , enum ,trait'
// These come in two forms 
// 1- Associated Function 
//  function that are defiend on type generally 
// a function that is tied to a type, but it doesn't operate on an instance of that type.
// It is typically called using the double colon syntax TypeName::function_name().
// The most common example is a constructor function, such as new.

// 2- methods 
//  is a function that is tied to an instance of a type. It takes self as its first parameter, representing the instance.
//  are associated functions that are called on a particular instance of a type.
// Methods can be called using the dot notation [ . ]



// Associated fun 

struct Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle {

    // Associated function that creates a new Rectangle
    fn new(width: u32, height: u32) -> Rectangle 
    {
     
     Rectangle { width, height}

    }

    // Method that calculate area of rectangle 
    fn area(&self) -> u32 {

        self.width * self.height  // Expression so no need for Return keyword
    }

    // Mutating Methods 
    // method can take &mut self as a parameter to mutate (change) the instance it operates on.
    // requires the caller object to be mutable

    fn resize_rect(&mut self,new_width:u32,new_height:u32) {
        println!("Resize Rectangle Dimensions");
        self.width = new_width;
        self.height = new_height;
    }

}

fn main() {
    
    let mut rect_ = Rectangle::new(50,100);

    println!("Rectangle coordinates are width {} and height {}",rect_.width ,rect_.height);

    println!("Area of Rectangle is {}",rect_.area());


    // println!("New Rectangle{:?} dimensions are width {} and height is {} and new area is {}",rect_.resize_rect(60,100),rect_.width,rect_.height,rect_.area());

    rect_.resize_rect(60,100);
    println!("New Rectangle dimensions are width {} and height is {} and new area is {}",rect_.width,rect_.height,rect_.area());

}
