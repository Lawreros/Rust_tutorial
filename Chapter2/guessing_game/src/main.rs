use std::io; //The io library comes from the standard library (which is known as io),
            //it provides many useful features, includeing accepting inputs
use rand::Rng;

fn main() {//empty parenthesies shown that there are no inputs
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new(); //makes the mutable variable "guess"
    //The :: syntax in the ::new line indicates that new is an associated function of the String type.
    //An associated function is implemented on a type, in this case String, rather than on a particular instance of a String.
    //Some languages call this a static method.

    io::stdin() //could also be written as std::io::stdin
        .read_line(&mut guess) //calls the read_line method to get input from user and append it into a string (without overwriting contents)
        .expect("Failed to read line");//what to print if io::Result returns an Err instead of an Ok value. 
        //If this instance of io::Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so you can use it.

    println!("You guessed: {}", guess);
}
