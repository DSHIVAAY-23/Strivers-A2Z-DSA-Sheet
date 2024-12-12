fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    // Split the array into two halves
    merge_sort(&mut arr[..mid]); // Sort the first half
    merge_sort(&mut arr[mid..]); // Sort the second half// Merge the sorted halves
    merge(arr, mid);
}
fn merge(arr: &mut [i32], mid: usize) {
    let mut left = arr[..mid].to_vec(); // Clone left half
    let mut right = arr[mid..].to_vec(); // Clone right half
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    // Merge the two halves
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    } // Copy any remaining elements from the left half
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    } // Copy any remaining elements from the right half
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn main() {
    let mut array = [3, 1, 4, 1, 5, 9, 2, 6];
    println!("Original array: {:?}", array);

    merge_sort(&mut array);
    println!("Sorted array: {:?}", array);
}
