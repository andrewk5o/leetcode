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
    fn test_max_area_sample() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(height), 49);
    }

    #[test]
    fn test_two_elements() {
        // For two bars, area = min(height[0], height[1]) * 1.
        let height = vec![1, 1];
        assert_eq!(max_area(height), 1);
    }

    #[test]
    fn test_all_same() {
        // All bars have the same height: area = height * (n - 1)
        let height = vec![5, 5, 5, 5];
        assert_eq!(max_area(height), 5 * 3);
    }

    #[test]
    fn test_increasing() {
        // In an increasing sequence, maximum area is between the first and last bars.
        let height = vec![1, 2, 3, 4, 5];
        assert_eq!(max_area(height), 6); // min(1,5) * 4 = 4, but better area is min(2,5) * 3 = 6
    }

    #[test]
    fn test_decreasing() {
        // In a decreasing sequence, maximum area is between the first and last bars.
        let height = vec![5, 4, 3, 2, 1];
        assert_eq!(max_area(height), 6); // min(5,1) * 4 = 4, but better area is min(5,2) * 3 = 6
    }
}
