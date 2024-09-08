use std::io;

fn main () {
    println!("Guess Number");

    println!("Please Enter The your Guess");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut  guess)
    .expect("failed to read line");

    println!("ur guess is {}",guess);

}


