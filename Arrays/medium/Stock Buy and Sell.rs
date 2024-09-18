impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
             let mut maxprofit = 0; // maximum sum (use i32::MIN to match the element type)
        let mut min = i32::MAX; 
             for &i in  &prices{
                 if i < min {
                     min = i
                 }
                 else {
                     maxprofit = std::cmp::max(maxprofit,i - min)
}
        }
        maxprofit
                
    }
}