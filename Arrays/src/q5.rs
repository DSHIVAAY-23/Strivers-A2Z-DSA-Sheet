use std::io;

pub fn m() {
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut cnt = 0;
    
    let number: i32 = input.trim().parse().expect("Please enter a valid number");
    
    println!("Divisors of {} are: ", number);
    for i in 1..=number {
        if number % i == 0 {
             println!("{}", i);
            cnt = cnt + 1;
        }
    }
    if cnt==2{
        println!("This is prime");
    }
    else{
        println!("This is not prime");
    }
}