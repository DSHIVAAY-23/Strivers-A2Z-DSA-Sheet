use crate::q1::largest_element;
use crate::q2::ndlargest_element;
use crate::q3::palindrome_number;
use crate::q4::amstrong_no;



mod q5;
mod q4;

mod q3;
mod q2;
mod q1;
fn main() {
    let array = [3, 5, 7, 2, 8, -1, 4, 10, 12];
    match ndlargest_element(&array) {
        Some(max) => println!("The largest element in the array is: {}", max),
        None => println!("The array is empty."),
    }

}
