fn main() {
    let arr = [2, 3,1,4, 6, 7, 8];
    let n = arr.len();
    let target = 7;
    for i in 0..n{
    for j in i+1..n{
        if arr[i] + arr[j] == target{
            println!("{} and {}", i,j);
            println!("{} and {} add up to {}", arr[i], arr[j], target);
            return;
        }
    }
    }
}