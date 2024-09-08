fn getsum(arr: &[i32]) -> i32 {
    let n = arr.len();
    
    if n == 0 {
        return 0;  // Base case: sum of an empty array is 0
    }
    
    if n == 1 {
        return arr[0];  // Base case: if there's only one element, return it
    }
    
    let output = getsum(&arr[1..]); // Recursive call with the rest of the slice
    let sum = arr[0] + output; // Add the first element to the sum of the rest
    
    return sum;
}

fn main() {
    let arr = [1, 2, 3, 4, 5];  // Example array
    let sum = getsum(&arr);  // Call the function with the array
    println!("Sum of array elements: {}", sum);  // Output the sum
}
