impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
     let mut maxi = i32::MIN; // maximum sum (use i32::MIN to match the element type)
        let mut sum = 0; // sum variable initialized to 0

        for &num in &nums { // iterate over elements, not indices
            sum += num;
            if sum > maxi {
                maxi = sum;
            }
            if sum < 0 { // if the current sum is less than 0, reset it
                sum = 0;
            }
        }
        maxi 
    }
}