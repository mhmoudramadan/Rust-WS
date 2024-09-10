
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

fn main() {
    let day_ = Week::saturday;
    println!("{:?}",day_);
}
