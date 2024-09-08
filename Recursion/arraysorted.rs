
fn sorted(arr:&[T],n:i32) -> bool {
    let mut n =arr.len();
    if n==0 && n==1 {return true} 

    if arr[0]<arr[1]{
        return true
    }
    sorted(arr+1,n-1)
}

fn main() {}