// 11. Container With Most Water
// https://leetcode.com/problems/container-with-most-water/

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(height), 49);
    }
}
