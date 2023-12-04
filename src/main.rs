use std::{thread, time::Duration, cmp::Ordering};
use rand::Rng;
use std::io;

pub fn wait(time: u64){
    thread::sleep(Duration::from_secs(time));
}

pub fn welcome_screen(){
    println!("Welcome to the guessing game show !");
    wait(2);
    println!("Where you have to guess what is the number I'm thinking of !!");
    wait(2);
}

fn main() {
    welcome_screen();
    let mut name = String::new();
    println!("let's start off by knowing your name : ");
    
    io::stdin()
    .read_line(&mut name)
    .expect("Cannot read value");
    while name.trim().is_empty() {
        println!("I'm sorry I couldn't get your name correctly, Could you repeat ?: ");
        io::stdin()
        .read_line(&mut name)
        .expect("Cannot read value");
    
}
    wait(1);
    println!("Nice to meet you {}", name.trim());
    wait(1);
    println!("Now let's begin !");
    wait(1);
    
    println!("Guess my number: I'm thinking between 0 to 50 :");
    
    let mut finish = false;
    let mut attempts = 1;
    let random_number: u64 = rand::thread_rng().gen_range(1..=50);
        

    while !finish {
        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Error while getting the number");
    
        let res = input.trim().parse::<u64>().expect("Provided data do not match with u64");
    
        match res.cmp(&random_number) {
        Ordering::Less => println!("My number is bigger !"),
        Ordering::Greater => println!("My number is smaller !"),
        Ordering::Equal =>{
            println!("Yes ! You won !");
            finish = true;
            wait(1);
            println!("You have successfully found my number {}, the number was {} and You have {} attempts", name.trim(), random_number, attempts);     
        },
    }
        attempts += 1;

    } 


}
