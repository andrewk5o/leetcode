// 15. 3Sum
// https://leetcode.com/problems/3sum/

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut res = vec![];

    nums.sort();

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let (mut l, mut r) = (i + 1, nums.len() - 1);
        while l < r {
            let sum = nums[i] + nums[l] + nums[r];
            if sum > 0 {
                r -= 1;
            } else if sum < 0 {
                l += 1;
            } else {
                res.push(vec![nums[i], nums[l], nums[r]]);
                l += 1;
                while l < r && nums[l] == nums[l - 1] {
                    l += 1;
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum_sample() {
        let mut result = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        // sort each inner vector and the overall vector so the order doesn't affect the comparison
        for triplet in &mut result {
            triplet.sort();
        }
        result.sort();
        let mut expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        for triplet in &mut expected {
            triplet.sort();
        }
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_three_sum_empty() {
        let result = three_sum(vec![]);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_three_sum_no_solution() {
        let result = three_sum(vec![1, 2, 3, 4]);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }
}
