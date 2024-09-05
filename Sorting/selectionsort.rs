fn main() {
let mut arr = vec![12,35,6,4,30,22];
let n = arr.len();

for i in 0..n-2{
    let mut mini = i;
    for j in i..n-1{
    if arr[j] <arr[mini]{ mini = j } 
    }
    let mut temp = arr[mini];
    mini = arr[i];
    arr[i] = temp;

    println!("Minimum element is {}", arr[i]);
}


}