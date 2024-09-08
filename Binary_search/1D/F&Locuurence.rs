    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
       fn binary_search(nums: &Vec<i32>, target: i32, find_first: bool) -> i32 {
            let (mut low, mut high) = (0, nums.len() as i32 - 1);
            let mut result = -1;
            
            while low <= high {
                let mid = (low + high) / 2;
                if nums[mid as usize] == target {
                    result = mid; // Found target, store the result
                    if find_first {
                        high = mid - 1; // Search on the left for the first occurrence
                    } else {
                        low = mid + 1; // Search on the right for the last occurrence
                    }
                } else if nums[mid as usize] < target {
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
            
            result
        }
        
        let start = binary_search(&nums,target,true);
         let start = binary_search(&nums, target, true);
        if start == -1 {
            return vec![-1, -1]; // If target is not found, return [-1, -1]
        }

        let end = binary_search(&nums, target, false);
        
        vec![start, end] 
    }

    fn main() {
        
    }