// 189. Rotate array
fn swap(nums: &mut Vec<i32>, l: &mut i32, r: &mut i32) {
    let (a, b) =
        (usize::try_from(*l).unwrap(), usize::try_from(*r).unwrap());
    (nums[a], nums[b]) = (nums[b], nums[a]);
    *l += 1;
    *r -= 1;
}

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let length = i32::try_from(nums.len()).unwrap();

    let k = k % length;

    let (mut l, mut r) = (0, length - 1);
    while l < r {
        swap(nums, &mut l, &mut r);
    }

    (l, r) = (0, k - 1);
    while l < r {
        swap(nums, &mut l, &mut r);
    }

    (l, r) = (k, length - 1);
    while l < r {
        swap(nums, &mut l, &mut r);
    }
}
