// 238. Product of Array Except Self
// https://leetcode.com/problems/product-of-array-except-self/

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![1; nums.len()];

    let mut prefix = 1;

    for i in 0..nums.len() {
        result[i] *= prefix;
        prefix *= nums[i];
    }

    let mut postfix = 1;

    for i in (0..nums.len()).rev() {
        result[i] *= postfix;
        postfix *= nums[i];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self() {
        let input = vec![1, 2, 3, 4];
        assert_eq!(product_except_self(input), vec![24, 12, 8, 6]);
    }
}
