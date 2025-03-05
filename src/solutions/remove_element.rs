// 27. Remove Element
// https://leetcode.com/problems/remove-element/

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }
    k as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut nums = vec![3, 2, 2, 3];
        let k = remove_element(&mut nums, 3);
        assert_eq!(k, 2);
        assert_eq!(&nums[..k as usize], &[2, 2]);
    }
}
