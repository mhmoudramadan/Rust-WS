// !! Traits
// * is a collection of methods defined for an unknown type: Self. They can access other methods declared in the same trait
// * traits define shared behavior between types, similar to interfaces in other languages. 
// * You can implement traits for various types, including structs, enums, and primitive types.
// * Traits are used to ensure types share common functionality, allowing polymorphism and code reuse.


struct Book {
    title : String,
    author: String,
}

trait Describable {
    fn describe(&self)->String;
}

impl Describable for Book {
    fn describe(&self)->String{
        format!("{} by {}", self.title, self.author)
    }
}

fn main() {
    println!("!!!! Hello, Traits !!!!");
    let book = Book {
        title: String::from("The Rust Book Guide"),
        author: String::from("Mahmoud ramadan"),
    };

    println!("{}", book.describe());
}
