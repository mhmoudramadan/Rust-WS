// Generic Bounded types

// You want to restrict a generic type to types that implement certain traits.

/// The function `print_any_G_value` in Rust prints out a generic value using the `Display` trait.
/// 
/// Arguments:
/// 
/// * `value`: The `print_any_G_value` function you provided takes a generic type `T` that implements
/// the `std::fmt::Display` trait as a parameter named `value`. This function then prints out the
/// value using the `println!` macro.

fn print_any_G_value<T : std::fmt::Display> (value:T) {
    println!("Generic Value is {}",value);
}

fn main() {
    print_any_G_value(42);
    print_any_G_value(44.2);
    print_any_G_value('a');
    print_any_G_value("Hello Generic bounded types");
}
