
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let max_trans = 2;
        let mut cache = vec![0; prices.len()];
        for trans in 0..max_trans {
            // best_by_in 维护了考虑前 N 次交易的最佳的买入点, 即 max(f[k-1, j] - prices[j]) { j in [0, i-1] }
            let mut best_buy_in = cache[0] - prices[0];
            for i in 1..prices.len() {
                // 复用 vec 前暂存一下前一次的计算结果
                let temp = cache[i];
                cache[i] = i32::max(cache[i - 1], best_buy_in + prices[i]);
                // 更新 best_buy_in
                best_buy_in = i32::max(best_buy_in, temp - prices[i]);
            }
        }
        return *cache.last().unwrap();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    }
}
