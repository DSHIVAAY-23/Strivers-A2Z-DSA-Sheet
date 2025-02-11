fn main(){
    let mut arr = [2,3,4,5,65,66,5,4,4];
    let n = arr.len();
    let result = sum(&arr);

    println!("{}",result);

   
}

fn sum(arr:&[i32])->i32{
    arr.iter().sum()
}