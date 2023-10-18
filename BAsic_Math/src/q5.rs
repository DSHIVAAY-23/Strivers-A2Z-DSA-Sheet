use std::io;

pub fn m() {
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let number: i32 = input.trim().parse().expect("Please enter a valid number");
    
    println!("Divisors of {} are: ", number);
    for i in 1..=number {
        if number % i == 0 {
            println!("{}", i);
        }
    }
}