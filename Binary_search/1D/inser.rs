    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        
        let mut low = 0;
        let mut high = (n-1) as i32;
         while (low<=high){
             let mut mid = (low +high)/2;
             if nums[mid as usize] ==target {return mid;}
             else if nums[mid as usize] <target {low = mid+1;}
             else {high = mid-1;}
        }
        low
    }
fn main() {
    
}