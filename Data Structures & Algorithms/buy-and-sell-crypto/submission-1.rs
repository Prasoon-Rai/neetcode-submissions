impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {

        let mut min_price = prices[0];
        let mut max_profit = 0;

        for price in prices {
            min_price = std::cmp::min(min_price, price);
            let profit = price - min_price;
            max_profit = std::cmp::max(profit, max_profit);
        }
        max_profit
    }
}