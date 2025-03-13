// 42. Trapping Rain Water Solution
// https://leetcode.com/problems/trapping-rain-water/

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap(height), 6);
    }
}
