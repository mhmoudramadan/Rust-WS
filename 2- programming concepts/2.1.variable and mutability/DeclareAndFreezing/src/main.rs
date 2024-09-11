
/// ###############

//  Declaration without initialiaztion 
// "seldom Used " . --> as it may lead to the use of uninitialized variables. 
// "Declare Variable and initialize it later"



///   #Freezing
/// When data is bound by the same name immutably it also freezes. 
/// Frozen data can't be modified until the immutable binding goes out of scope:

 
fn main() {

    println!("###### Declare Without Init ########");

    let variable_1 ; // Declare without initialization

    {
        let x = 15;

        variable_1 = x + 10 ;  // Initialize the Variable
    }
    
    println!("Variable is {}" ,variable_1);

    let variable_2 ; 

    // println!("Var before init is {}", variable_2); // Error 

    variable_2= String::from("Mahmoud");

    println!("Var before init is {}", variable_2); 

    //  Freezing 

    println!("###### Freezing ########");

    let mut unfreez_var = 100u32;

    println!("Variable before Enter inner scope is {}",unfreez_var); // output 100
    {
        // shadowing by immutable Variable

        let unfreez_var = unfreez_var;

         /// Error found 
         /// "unfreez_var --> is frozen in this scope" 
         /// the frozen var is free when it get outof scope because shadowing here by default is immutable
         /// 
         // unfreez_var = 30;
         println!("Variable after shadowing in  inner scope is {}",unfreez_var); // output 100
        }

    unfreez_var = 500;
    println!("unfrozen after exit inner scope is {}",unfreez_var); // output 500
}
