// Traits can also be generic, allowing them to define behavior for multiple types.

// The Generics trait is a fundamental trait in Rust that defines the behavior of generic types.
// It provides methods for accessing and manipulating the type parameters of a generic type.
// By implementing the Generics trait for your generic types, you can gain more control over their behavior and interactions.

// Key Methods of the Generics Trait:

// type_name: Returns the name of the generic type parameter.
// type_of: Returns the concrete type of the generic type parameter at runtime.
// has_type_parameter: Checks if the generic type has a type parameter.
// num_type_parameters: Returns the number of type parameters in the generic type.


#[derive(Debug)]
/// The `Article` struct in Rust represents an article with a headline and content.
/// 
/// Properties:
/// 
/// * `headline`: The `headline` property in the `Article` struct represents the title or main heading
/// of the article. It typically provides a brief summary or overview of the content that follows.
/// * `content`: The `content` property in the `Article` struct represents the main text or body of the
/// article. It typically contains the detailed information, story, or message that the article conveys
/// to the readers.
struct Article {
    headline: String,
    content: String,
}


/// The `trait Summary<T>` defines a trait named `Summary` that is generic over a type `T`. This trait
/// specifies a method `summarize` that takes a reference to `self` and returns a value of type `T`.
trait Summary<T> {
    fn summarize(&self) -> T;
}



/// This code snippet is implementing the `Summary` trait for the `Article` struct with the type
/// parameter `String`.
impl Summary<String> for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }
}

fn main() {

    let article = Article {
        headline: String::from("Generic Traits"),
        content : String::from("demonstrate how to implementation it  "),
    };

    println!("Summary is {}",article.summarize());
    
}
