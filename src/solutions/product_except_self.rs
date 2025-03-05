// 238. Product of Array Except Self
use std::collections::HashMap;

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

pub fn two_sum_2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
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

pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut l, mut r): (usize, usize) = (0, height.len() - 1);
    let mut area = 0;

    while l < r {
        if height[l] < height[r] {
            area = area.max(height[l] as usize * (r - l));
            l += 1;
        } else {
            area = area.max(height[r] as usize * (r - l));
            r -= 1;
        }
    }
    area as i32
}

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

pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    if gas.iter().sum::<i32>() < cost.iter().sum::<i32>() {
        return -1;
    }

    let mut fuel = 0;
    let mut result = 0;

    for i in 0..gas.len() {
        fuel += gas[i] - cost[i];
        if fuel < 0 {
            fuel = 0;
            result = i + 1;
        }
    }

    result as i32
}

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

pub fn trap(height: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0, height.len() - 1);
    let (mut max_left, mut max_right) = (height[l], height[r]);
    let mut result = 0;

    while l < r {
        if max_left < max_right {
            l += 1;
            max_left = max_left.max(height[l]);
            result += max_left - height[l];
        } else {
            r -= 1;
            max_right = max_right.max(height[r]);
            result += max_right - height[r];
        }
    }

    result
}
