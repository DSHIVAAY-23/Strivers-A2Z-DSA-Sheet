
pub struct Solution {}


impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        1.0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: implementation
    #[test]
    #[ignore]
    fn test_4() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
