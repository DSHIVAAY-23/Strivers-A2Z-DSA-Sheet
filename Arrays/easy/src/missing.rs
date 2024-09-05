impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        
     let n = nums.len() as i32;
        let sum = (n*(n+1))/2;
        let mut sum1 = 0;
        for i in 0..nums.len(){
            sum1 +=nums[i] as i32;
        }
        let num  = sum - sum1;
        return num as i32;
    
    }
}