fn swap(a: &mut i32, b: &mut i32){
    if a!=b {
        *a= *a^*b;
        *b= *a^*b;
        *a= *a^*b;

    }
}


fn main(){
    let mut a = 5;
    let mut b = 9;
    println!("Before swapping: a = {}, b = {}", a, b);
    
    swap(&mut a, &mut b);
    
    println!("After swapping: a = {}, b = {}", a, b);


    println!("Aft

}