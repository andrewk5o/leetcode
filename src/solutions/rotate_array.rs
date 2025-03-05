// 189. Rotate Array
// https://leetcode.com/problems/rotate-array/

pub fn swap(nums: &mut Vec<i32>, l: i32, r: i32) {
    let (l, r) = (l as usize, r as usize);
    (nums[l], nums[r]) = (nums[r], nums[l]);
}

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let length = nums.len() as i32;

    let k = k % length;

    let (mut l, mut r) = (0, length - 1);

    while l < r {
        swap(nums, l, r);
        l += 1;
        r -= 1;
    }

    (l, r) = (0, k - 1);
    while l < r {
        swap(nums, l, r);
        l += 1;
        r -= 1;
    }

    (l, r) = (k, length - 1);
    while l < r {
        swap(nums, l, r);
        l += 1;
        r -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }
}
