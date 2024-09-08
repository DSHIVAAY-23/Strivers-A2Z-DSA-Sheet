pub fn search(nums: &Vec<i32>, target: i32, start: i32, end: i32) -> i32 {
    // Base case: if start index exceeds end, return -1 (target not found)
    if start > end {
        return -1;
    }

    // Calculate the middle index
    let mid = (start + end) / 2;

    // Check if the middle element is the target
    if nums[mid as usize] == target {
        return mid;  // Return the index of the target
    }

    // If the middle element is less than the target, search the right half
    if nums[mid as usize] < target {
        return search(nums, target, mid + 1, end);
    }

    // If the middle element is greater than the target, search the left half
    return search(nums, target, start, mid - 1);
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]; // Example sorted array
    let target = 5;
    let result = search(&nums, target, 0, (nums.len() as i32) - 1);
    
    if result != -1 {
        println!("Target found at index: {}", result);
    } else {
        println!("Target not found");
    }
}
