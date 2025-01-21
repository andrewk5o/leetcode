// 28. Remove duplicates from sorted array
pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k: usize = 1;
        let mut curr = nums[0];
        for n in 1..nums.len() {
            if nums[n] != curr {
                nums[k] = nums[n];
                k += 1;
                curr = nums[n];
            }
        }
        return i32::try_from(k).unwrap();
    }
}
