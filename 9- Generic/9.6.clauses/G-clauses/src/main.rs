// where clauses 
// allow you to specify constraints on generic types more cleanly and concisely,
//  especially when there are multiple constraints or when the function signature becomes complex. 
// Instead of placing all trait bounds in the function's declaration, the where clause enables you to define them separately


// A bound can also be expressed using a where clause immediately before the opening {, rather than at the type's first mention. 
// Additionally, where clauses can apply bounds to arbitrary types, rather than just to type parameters.


// basic bound without where clause

fn print_values_without_clause<T : std::fmt::Display + std::fmt::Debug> (t:T,a:T) {
    println!("t is {} and a is {:?}",t,a);
}


// basic bound with where clause
fn print_values_with_clause<T>(t:T,c:T) 
    where
        T : std::fmt::Display + std::fmt::Debug ,
        {
            println!("t is {} and c is {:?}",t,c);
        }


// basic bound with where clause in structs

/// The `struct container<T, U>` declaration with the where clause is defining a generic struct named
/// `container` with two type parameters `T` and `U`. The where clause specifies constraints on these
/// type parameters: `T` must implement the `Debug` trait, and `U` must implement the `Display` trait.
struct container<T,U>
    where
        T:std::fmt::Debug,
        U:std::fmt::Display, 
        {
            val1:T,
            val2:U,
        }


/// The `impl<T,U> container<T,U>` block with the where clause is implementing methods for the
/// `container` struct. It specifies that for any generic types `T` and `U`, where `T` implements the
/// `Debug` trait and `U` implements the `Display` trait, the methods defined within this block will be
/// available for instances of the `container` struct with those specific type constraints.
impl<T,U> container<T,U>
        where
            T:std::fmt::Debug,
            U:std::fmt::Display, 
            {
                fn new(val1:T,val2:U)->Self{
                    Self{val1,val2}
                }

                fn display_struct(&self){
                    println!("Value1: {:?}, value2: {}", self.val1, self.val2);
                }
            }


fn main() {
    println!("Hello, where clauses!");

    print_values_without_clause('m','a');
    print_values_with_clause(300,200);


    let my_con = container::new(100,"Rust language");
    my_con.display_struct();
}
