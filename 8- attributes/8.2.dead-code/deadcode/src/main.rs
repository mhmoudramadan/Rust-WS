
// The compiler provides a dead_code lint that will warn about unused functions.
//  An attribute can be used to disable the lint.

fn used_fn(){

    println!("this function is used");
}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_fn(){

}

fn main() {

    used_fn();
}
