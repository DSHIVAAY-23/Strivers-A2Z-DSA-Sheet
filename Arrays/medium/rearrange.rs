impl Solution {
    pub fn rearrange_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mut pos = Vec::new();
        let mut neg = Vec::new();
        
    for &i in &nums {
        if i>0 {
            pos.push(i);
        }
        else{
            neg.push(i);
        }
         
        
        }
        for i in 0..(nums.len())/2 {
            nums[2*i] = pos[i];
            nums[2*i+1] =neg[i];
        }
        nums
        
    }
}