// 45. Jump Game II

pub fn jump(nums: Vec<i32>) -> i32 {
    let mut jumps = 0;
    let (mut left, mut right) = (0, 0);
    while right < nums.len() - 1 {
        let mut farthest = 0;
        for i in left..=right {
            farthest = farthest.max(i + nums[i] as usize);
        }
        left = right + 1;
        right = farthest;
        jumps += 1;
    }
    jumps
}
