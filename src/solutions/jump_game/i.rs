// 55. Jump Game
// https://leetcode.com/problems/jump-game/

pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut goal = nums.len() - 1;

    for i in (0..nums.len() - 1).rev() {
        if i + nums[i] as usize >= goal {
            goal = i;
        };
    }

    goal == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_jump() {
        assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
    }

    #[test]
    fn test_single_element() {
        // With one element, we are already at the goal.
        assert_eq!(can_jump(vec![0]), true);
    }
}
