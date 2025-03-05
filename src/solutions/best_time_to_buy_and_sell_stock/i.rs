// 121. Best Time to Buy and Sell Stock

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut p = 0;
    let (mut l, mut r): (usize, usize) = (0, 1);

    while r < prices.len() {
        if prices[r] > prices[l] {
            p = p.max(prices[r] - prices[l]);
        } else {
            l = r;
        }
        r += 1;
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
    
    #[test]
    fn test_single_day() {
        // With one day, no transaction is possible.
        assert_eq!(max_profit(vec![5]), 0);
    }
}
