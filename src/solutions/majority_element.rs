// 169. Majority Element
// https://leetcode.com/problems/majority-element/

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut curr = nums[0];
    let mut k = 1;

    for n in 1..nums.len() {
        k = if nums[n] == curr { k + 1 } else { k - 1 };
        if k == 0 {
            curr = nums[n];
            k = 1;
        }
    }

    curr as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
    }
}
