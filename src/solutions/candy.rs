// 135. Candy
// https://leetcode.com/problems/candy/

pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut candies = vec![1; ratings.len()];

    for i in 1..ratings.len() {
        if ratings[i] > ratings[i - 1] {
            candies[i] = candies[i - 1] + 1;
        }
    }

    for i in (0..ratings.len() - 1).rev() {
        if candies[i] > candies[i + 1] {
            continue;
        };

        if ratings[i] > ratings[i + 1] {
            candies[i] = candies[i + 1] + 1;
        }
    }

    candies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_candy() {
        assert_eq!(candy(vec![1, 0, 2]), 5);
    }
}
