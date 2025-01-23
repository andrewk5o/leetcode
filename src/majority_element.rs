// 26. Remove duplicates from sorted array
// Boyer–Moore majority vote algorithm
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut curr = nums[0];
    let mut k = 1;
    for n in 1..nums.len() {
        if nums[n] == curr {
            k += 1;
        } else {
            k -= 1;
        }
        if k == 0 {
            curr = nums[n];
            k = 1;
        }
    }
    i32::try_from(curr).unwrap()
}
