
// enumerations
// are a way to define a type by enumerating its possible variants. 
// Each variant can either be simple or hold data, 
// making enums powerful for representing various states or configurations in a concise way.

//  basic Enum
// A simple enum without associated data. Each variant is a distinct value.

#[derive(Debug)]
enum Week{
    saturday,
    sunday,
    monday,
    tuesday,
    wednesay,
    thursday,
    friday,
}

//  Enum with Associated Data:
// Enum variants can also hold data, similar to a struct. This allows you to associate values with each variant
#[derive(Debug)]
enum Shape{
    Rectangle(f32,f32), // width and height
    Circle(f32), // raduis 
}



// Enum with Named Fields:
#[derive(Debug)]
enum Vechile{
    car{manufacturer:String ,model:String,year:u16},
    motorcycle{manufacturer:String ,model:String,capacity:u32},
}


// Match keyword
// When an enum variant contains associated data, you can extract the data inside the match block.

fn vechile_Info(vechile:Vechile){
    match vechile {
        Vechile::car{ manufacturer,model,year} =>{
            println!("{} {} {}",manufacturer,model,year);
        }
        Vechile::motorcycle{manufacturer,model,capacity} =>{
            println!("{} {} {} ",manufacturer,model,capacity);
        }
    }
}


fn main() {
    let day_ = Week::saturday;
    println!("{:?}",day_);

    let circle = Shape::Circle(55.9);
    let rec = Shape::Rectangle(3.6,4.9);

    println!("Raduis of {:?}",circle);
    println!("dimension of  {:?}",rec);

    let car_1 = Vechile::car{manufacturer:String::from("Toyota"),
                           model:String::from("corrola"),
                            year:2024,
                        };

    let motorcycle_1  = Vechile::motorcycle{
                                            manufacturer:String::from("MCV"),
                                            model:String::from("Heavy Truck"),
                                            capacity:2000,
    };              
    
    println!("Specification of {:?}",car_1);
    println!("Specification of  {:?}",motorcycle_1);
    vechile_Info(car_1);
    vechile_Info(motorcycle_1);



    //  use --> keyword
    // Usage : import the variants into scope and use them directly without needing to prefix them with the enum name.
    
    // Ways of usage
    // 1
    use Week::{saturday,sunday,monday,tuesday,wednesay,thursday,friday};

    let day_1= saturday;

    // 2 
    // // Automatically `use` each name inside `Shape` and weeks.
    use crate::Shape::*; 
    let rec_2 = Rectangle(2.0,4.0);
    println!("{:?} is ",rec_2);

    use crate::Week::*;
    let thr = tuesday;
    println!("{:?} ",thr);

}
