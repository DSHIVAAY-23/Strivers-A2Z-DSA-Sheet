// array is sorted or not sorted

pub fn sorted(arr:&[i32]) ->bool{
  let mut sorted = true;
  for i in 0..arr.len()-1{
    if arr[i] > arr[i+1]{
      sorted = false;
      break;
    }
  }
  sorted
}
fn main() {
  let array = [3, 5, 7, 2, 8, -1, 4, 10, 12];
  sorted(&array);
  println!("The array is sorted: {}", sorted(&array));
}
