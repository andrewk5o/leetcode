// 167. Two Sum II - Input Array Is Sorted
// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut l, mut r) = (0, numbers.len() - 1);

    while l < r {
        let sum = numbers[l] + numbers[r];

        if sum == target {
            return vec![l as i32 + 1, r as i32 + 1];
        }

        if sum < target {
            l += 1;
        } else {
            r -= 1;
        }
    }

    vec![0, 0]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum_ii() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(numbers, target), vec![1, 2]);
    }
}
