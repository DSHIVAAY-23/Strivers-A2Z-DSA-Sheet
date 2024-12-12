
fn sum(s:Vec<i32>) -> i32{
    let x:i32 = s.iter().sum();
    x
}
fn main(){
    let x  = vec![2,3,4,5,6];
    let z:i32 = sum(x);
     println!("Sum: {}", z);
    let t  = vec![2,3,4,5,6];

    let y: Vec<_> = t.iter().map(|t|t+1).collect();
  //  let total :i32 = y.sum();
  println!("Sum: {:?}", y);
}