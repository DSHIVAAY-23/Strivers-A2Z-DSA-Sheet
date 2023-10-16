//reverse number -- take a reverse no to stoe the number

pub fn reverse_no(n:u32) -> u32 {
    let mut x  = n;
    let mut reverse_no  = 0;
    

    while x!=0 {

     let   digit = x%10;
       reverse_no = (reverse_no*10) + digit ;
       x = x/10;
       


    }
    reverse_no
}