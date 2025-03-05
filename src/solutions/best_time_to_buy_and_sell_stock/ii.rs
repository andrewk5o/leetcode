// 122. Best Time to Buy and Sell Stock II
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut p = 0;

    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            p += prices[i] - prices[i - 1];
        }
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_single_day() {
        // With one day, no profit can be made.
        assert_eq!(max_profit(vec![5]), 0);
    }
}
