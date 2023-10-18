use crate::q1::count_digits;
use crate::q2::reverse_no;
use crate::q3::palindrome_number;
use crate::q4::amstrong_no;



mod q5;
mod q4;

mod q3;
mod q2;
mod q1;
fn main() {
    println!("Hello, world!");
    



    println!("The number of digits in {}", count_digits(12340));

    println!("The  {}", reverse_no(1234));
    println!("  {}", palindrome_number(121));
    println!("  {}", amstrong_no(37));
    

    q5::m();


}
