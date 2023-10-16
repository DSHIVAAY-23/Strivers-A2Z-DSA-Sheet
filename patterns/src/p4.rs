pub fn p4(){
    let  x = 5;
    let y =5;
    for _i in 0..x {
        for _j in 0.._i{
            print!(" ");
         }
        
       for _j in 0..2*y - (2*_i +1){
       print!("*");
      }
      for _j in 0.._i{
             print!(" ");
          }
          println!()
      }
      
  }