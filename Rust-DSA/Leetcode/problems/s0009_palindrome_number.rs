

// TODO: not optimal, we only have to revert half of the string
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut digits: Vec<i32> = Vec::new();
        let mut input = x;
        while input != 0 {
            digits.push(input % 10);
            input = input / 10;
        }
        let len = digits.len();
        // handle one digit
        if len < 2 {
            return true;
        }
        // handle end with 0
        if digits[0] == 0 {
            return false;
        }
        let mut i = 0;
        while i < len / 2 {
            if digits[i] != digits[len - 1 - i] {
                return false;
            }
            i += 1;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        assert_eq!(Solution::is_palindrome(-32), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(0), true);
        assert_eq!(Solution::is_palindrome(9), true);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(2222), true);
        assert_eq!(Solution::is_palindrome(11222211), true);
    }
}
