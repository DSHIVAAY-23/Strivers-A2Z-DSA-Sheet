pub fn p3(){
  let  x = 5;
  let y =5;
  for _i in 0..x {
     for _j in 0..y - _i +1{
         print!(" ");
    }
    for _j in 0..2*_i +1{
           print!("*");
        }
        println!()
    }
    
}