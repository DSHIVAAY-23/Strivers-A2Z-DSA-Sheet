
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 1 {
            return len as i32;
        }
        let mut slow = 0usize;
        for fast in 1..len {
            if nums[slow] != nums[fast] {
                slow += 1;
                nums[slow] = nums[fast];
            }
        }
        nums.truncate(slow + 1);
        (slow + 1) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_26() {
        assert_eq!(Solution::remove_duplicates(&mut vec![]), 0);
        let mut vec1 = vec![1, 1, 1, 1, 3];
        assert_eq!(Solution::remove_duplicates(&mut vec1), 2);
        assert_eq!(vec1, vec![1, 3]);
        let mut vec2 = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut vec2), 2);
        assert_eq!(vec2, vec![1, 2]);
    }
}
