
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut curr = 0;
        for i in 1..prices.len() {
            curr = curr + prices[i] - prices[i - 1];
            if curr <= 0 {
                curr = 0;
            } else {
                max = i32::max(max, curr);
            }
        }
        max
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_121() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
