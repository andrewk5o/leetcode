// 26. Remove duplicates from sorted array

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k: usize = 1;
    for n in 1..nums.len() {
        if nums[n] != nums[n - 1] {
            nums[k] = nums[n];
            k += 1;
        }
    }
    i32::try_from(k).unwrap()
}
