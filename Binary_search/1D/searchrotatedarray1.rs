    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len() as i32 - 1;
        
        while low <= high {
            let mid = (low + high) / 2;
            
            if nums[mid as usize] == target {
                return mid as i32; 
            }
            
            if nums[low as usize] <= nums[mid as usize] {
                if target >= nums[low as usize] && target < nums[mid as usize] {
                    high = mid - 1;
                } else {
                    low = mid + 1; 
                }
            } 
            else {
                if target > nums[mid as usize] && target <= nums[high as usize] {
                    low = mid + 1; 
                } else {
                    high = mid - 1; 
                }
            }
        }
        
        -1
    }


    fn main() {
        
    }