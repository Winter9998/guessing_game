// Importing necessary standard library modules and external crates
use std::{thread, time::Duration, cmp::Ordering};
use rand::Rng; // Random number generator
use std::io; // Input/Output library

// Function to pause execution for a specified number of seconds
pub fn wait(time: u64){
    thread::sleep(Duration::from_secs(time));
}

// Function to display the welcome message at the beginning of the game
pub fn welcome_screen(){
    println!("Welcome to the guessing game show !");
    wait(2);
    println!("Where you have to guess what is the number I'm thinking of !!");
    wait(2);
}

fn main() {
    welcome_screen();

    // Store the player's name
    let mut name = String::new();
    println!("let's start off by knowing your name : ");
    
    // Reading player's name from standard input
    io::stdin()
    .read_line(&mut name)
    .expect("Cannot read value");

    // Loop until a valid name is entered
    while name.trim().is_empty() {
        println!("I'm sorry I couldn't get your name correctly, Could you repeat ?: ");
        io::stdin()
        .read_line(&mut name)
        .expect("Cannot read value");
    }

    // Greet the player
    wait(1);
    println!("Nice to meet you {}", name.trim());
    wait(1);
    println!("Now let's begin !");
    wait(1);
    
    println!("Guess my number: I'm thinking between 0 to 50 :");
    
    // Game variables
    let mut finish = false;
    let mut attempts = 1;
    let random_number: u64 = rand::thread_rng().gen_range(1..=50); // Random number between 1 and 50
        
    // Main game loop
    while !finish {
        let mut input = String::new();

        // Reading player's guess from standard input
        io::stdin()
        .read_line(&mut input)
        .expect("Error while getting the number");
    
        // Parsing the input to a number and handling invalid input
        let res = input.trim().parse::<u64>().expect("Provided data do not match with u64");
    
        // Comparing the guess to the random number
        match res.cmp(&random_number) {
        Ordering::Less => println!("My number is bigger !"),
        Ordering::Greater => println!("My number is smaller !"),
        Ordering::Equal =>{
            // If the guess is correct
            println!("Yes ! You won !");
            finish = true;
            wait(1);
            println!("You have successfully found my number {}, the number was {} and You had {} attempts", name.trim(), random_number, attempts);     
        },
    }
        attempts += 1; // Incrementing the number of attempts

    } 
}
