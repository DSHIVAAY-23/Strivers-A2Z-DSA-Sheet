fn main(){
    println!("Binary: {:08b}", 59);

    let mut a =59;
    let mut i = 4;
    let mut b = a & !((1<<i+1) -1);
    let mut c = a & ((1<<i+1) -1);

    println!("lsb: {:08b}", b);
    println!("msb: {:08b}", c);


}