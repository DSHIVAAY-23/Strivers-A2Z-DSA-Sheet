//palindrome number -- just take a duplicate and check it with palindrome number

pub fn palindrome_number(n:u32) -> bool{
    let mut x  = n;
    let mut palindrome_number  = 0;
    let mut dub = n;
    

    while x!=0 {

     let   digit = x%10;
     palindrome_number = (palindrome_number*10) + digit ;
       x = x/10;
       


    }
     dub == palindrome_number
}