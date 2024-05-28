//amstrong n0  -- 

pub fn amstrong_no(n:u32) -> bool{
    let mut x  = n;
    let mut sum = 0;
    let mut dub = n;
    

    while x!=0 {

     let   digit = x%10;
       sum  = sum  + (digit*digit*digit); 
       x = x/10;
       


    }
    sum == dub
}