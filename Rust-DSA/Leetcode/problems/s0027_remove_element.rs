

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() < 1 {
            return 0;
        }
        let (mut start, mut end) = (0_usize, nums.len() - 1);
        while start < end {
            if nums[start] == val {
                nums[start] = nums[end - 1];
                end -= 1;
            } else {
                start += 1;
            }
        }
        end as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_27() {
        let mut vec1 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut vec1, 2), 5);
        assert_eq!(vec1[0..5], [0, 1, 4, 0, 3]);
        assert_eq!(Solution::remove_element(&mut vec![], 2), 0);
        assert_eq!(
            Solution::remove_element(&mut vec![1, 2, 2, 2, 2, 2, 2], 2),
            1
        );
        assert_eq!(
            Solution::remove_element(&mut vec![2, 2, 2, 2, 2, 2, 2], 2),
            0
        );
        assert_eq!(Solution::remove_element(&mut vec![1], 1), 0);
    }
}
