//use keyword for imports which are inbuilt as well as 
//the dependencies included in the cargo.toml file
use rand::Rng ;
use std::cmp::Ordering;
use std::io;
//Main function just like C++ etc first function
//to be executed during the build
fn main() {
    println!("Guess the number!");
    //Type safety to maintain better error management in case of 
    //compile time erors 
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //loop for iterating this one is infinite 
    loop {
        //standard io operation
        println!("Please input your guess.");
        //mutable datatypes to be declared by keyword mut
        //by default they are immutable in case of RUST and rust also manages the memory 
        //by borrow and responsibility method instead of standard in case of C or C++
        let mut guess = String::new();
        //Input reading along with typesafety and by default can be accessed directly 
        //using pass by reference 
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        //control flow in case of RUST  just like the 
        //switch case in case of C++
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}