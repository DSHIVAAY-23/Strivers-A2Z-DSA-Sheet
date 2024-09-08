
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
         let (mut low, mut high) = (0, nums.len() as i32 - 1);
        
        while low <= high {
            let mid = (low + high) / 2; 
            
            if nums[mid as usize] == target {
                return mid; 
            } else if nums[mid as usize] < target {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        
        -1 
    }
        
    fn main(){
        
    }
