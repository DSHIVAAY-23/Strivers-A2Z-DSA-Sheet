// Problem Statement: Given an array, we have to find the largest element in the array.



pub fn largest_element(arr:&[i32]) -> Option<i32> {
   let mut max = 0;
   for i in 0..arr.len() {
       if arr[i] > max {
           max = arr[i];
       }
   }

   Some(max)
}