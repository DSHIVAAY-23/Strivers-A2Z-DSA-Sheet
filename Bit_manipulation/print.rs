fn print_binary(num: i32) {
    println!("Binary: {:08b}", num);
}

fn main() {
    print_binary(9); // This will print: "Binary: 00000101"
    let mut a= 9;
    let mut i = 2;
    if (a & (1 <<i)!=0){
        println!(" ith bit is set");
    }
    else {
        println!(" is not set");
    }
    //bit set
    println!("{:?}", a | 1<<3);
    println!("set: {:08b}", a | (1<<i));


    //unset ith bit

    // println!("Binary: {:08b}", !a);
    println!("unset: {:08b}", a & !(1<<3));
    //toggle
    println!("toggled: {:08b}", a ^ (1<<3));

    let mut count = 0;
    for i in 0..31{
        if (a & (1 << i)!=0){
            count += 1;
        }
    }
    println!("Number of set bits: {}", count);
    println!("multiply :{}",a<<1);
    println!("divide :{}", a>>1);


    let mut B = 'B' as u8;
    let mut b = B | (1<<5);
    println!("{:?}", b as char);
    let mut C = 'C' as u8;
    let mut c = C | (' ' as u8);
    println!("{:?}", c as char);

}
