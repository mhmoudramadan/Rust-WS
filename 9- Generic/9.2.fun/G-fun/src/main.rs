// The same set of rules can be applied to functions: a type T becomes generic when preceded by <T>.
// Using generic functions sometimes requires explicitly specifying type parameters
// This may be the case if the function is called where the return type is generic,
// or if the compiler doesn't have enough information to infer the necessary type parameters

#[derive(Debug)]
struct A;          // Concrete type `A`.
#[derive(Debug)]
struct Single(A);       // Concrete type `Single`.
#[derive(Debug)]
struct SingleGen<T>(T); // Generic type `SingleGen`.

// The following functions all take ownership of the variable passed into
// them and immediately go out of scope, freeing the variable.

// Define a function `reg_fn` that takes an argument `_s` of type `Single`.
// This has no `<T>` so this is not a generic function.
fn reg_fn(_s: Single) {
    println!("{:?}",_s);
}

// Define a function `gen_spec_t` that takes an argument `_s` of type `SingleGen<T>`.
// It has been explicitly given the type parameter `A`, but because `A` has not 
// been specified as a generic type parameter for `gen_spec_t`, it is not generic.
fn gen_spec_t(_s: SingleGen<A>) {
    println!("{:?}",_s);
}

// Define a function `gen_spec_i32` that takes an argument `_s` of type `SingleGen<i32>`.
// It has been explicitly given the type parameter `i32`, which is a specific type.
// Because `i32` is not a generic type, this function is also not generic.
fn gen_spec_i32(_s: SingleGen<i32>) {
    println!("{:?}",_s);
}

// Define a function `generic_fn` that takes an argument `_s` of type `SingleGen<T>`.
// Because `SingleGen<T>` is preceded by `<T>`, this function is generic over `T`.
fn generic_fn<T: std::fmt::Debug>(_s: SingleGen<T>) {
    println!("{:?}",_s);
}

fn main() {
    // Using the non-generic functions
    reg_fn(Single(A));          // Concrete type.
    gen_spec_t(SingleGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SingleGen(6)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic_fn()`.
    generic_fn::<char>(SingleGen('a'));

    // Implicitly specified type parameter `char` to `generic_fn()`.
    generic_fn(SingleGen('c'));
}
