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
