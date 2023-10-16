// Count digits in a number
//Problem Statement: Given an integer N, write a program to count the number of digits in N.

pub fn count_digits(n:u32) -> u32 {
    let mut x  = n;
    let mut count  = 0;

    while x!=0 {
       x /= 10 ;
       count += 1;


    }
    count
    

}