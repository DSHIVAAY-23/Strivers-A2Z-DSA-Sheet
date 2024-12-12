

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry = 0;
        for i in (0..digits.len()).rev() {
            digits[i] = if digits[i] == 9 {
                carry = 1;
                0
            } else {
                carry = 0;
                digits[i] + 1
            };
            if carry == 0 {
                break;
            }
        }
        if carry > 0 {
            digits.insert(0, 1);
        }
        digits
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_66() {
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
        assert_eq!(Solution::plus_one(vec![9, 9, 9, 9]), vec![1, 0, 0, 0, 0]);
        assert_eq!(
            Solution::plus_one(vec![1, 0, 9, 9, 9, 9]),
            vec![1, 1, 0, 0, 0, 0]
        );
    }
}
