use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hashmap = HashMap::new();

    for i in 0..nums.len() {
        if hashmap.contains_key(&nums[i]) {
            return vec![hashmap[&nums[i]] + 1, i as i32 + 1];
        }

        hashmap.insert(target - nums[i], i as i32);
    }

    vec![0, 0]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum_i() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![1, 2]);
    }
}
