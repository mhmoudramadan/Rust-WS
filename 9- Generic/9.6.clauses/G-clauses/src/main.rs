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

fn print_values_with_clause<T>(t:T,c:T) 
    where
        T : std::fmt::Display + std::fmt::Debug ,
        {
            println!("t is {} and c is {:?}",t,c);
        }

fn main() {
    println!("Hello, where clauses!");

    print_values_without_clause('m','a');
    print_values_with_clause(300,200);
}
