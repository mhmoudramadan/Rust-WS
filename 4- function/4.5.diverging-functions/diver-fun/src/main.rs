// Diverging functions 'DV'
// never return. They are marked using [ ! ], which is an empty type.
// and instead cause the program to either loop indefinitely or terminate (e.g., with a panic or exit).

// Key Points about Diverging Functions:

//1     Return type !: A diverging function never returns, so its return type is ! (pronounced "never").
//2     Indefinite loop or panic: The function might loop forever, panic, or exit the program, but it will never return a value.
//3     Use case: Diverging functions can be useful in situations where returning a value is not possible or
//              desired( error handling, termination).

// Syntax

// fn diverging_function() -> ! {
//     // The function doesn't return anything
//     panic!("This function never returns!");
// }

fn main() {
   
}
