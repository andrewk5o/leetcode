mod best_time_to_buy_stock;
mod majority_element;
mod merge_sorted_array;
mod remove_duplicates_from_sorted_array;
mod remove_duplicates_from_sorted_array_ii;
mod remove_element;
mod rotate_array;

fn main() {
    struct Solution;

    impl Solution {
        fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
            merge_sorted_array::merge(nums1, m, nums2, n);
        }

        fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
            remove_element::remove_element(nums, val)
        }

        fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            remove_duplicates_from_sorted_array::remove_duplicates(nums)
        }

        fn remove_duplicates_ii(nums: &mut Vec<i32>) -> i32 {
            remove_duplicates_from_sorted_array_ii::remove_duplicates(nums)
        }

        fn rotate_array(nums: &mut Vec<i32>, k: i32) {
            rotate_array::rotate(nums, k);
        }

        fn max_profit(prices: Vec<i32>) -> i32 {
            best_time_to_buy_stock::max_profit(prices)
        }

        fn majority_element(nums: Vec<i32>) -> i32 {
            majority_element::majority_element(nums)
        }
    }

    Solution::merge(&mut vec![1, 2, 3, 0, 0, 0], 3, &mut vec![2, 5, 6], 3);
    Solution::remove_element(&mut vec![3, 2, 2, 3], 3);
    Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]);
    Solution::remove_duplicates_ii(&mut vec![
        0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 3, 3, 4,
    ]);
    Solution::majority_element(vec![2, 2, 2, 1, 2, 1, 2, 2, 1, 2, 2, 3, 3, 4]);
    Solution::rotate_array(&mut vec![1, 2, 3, 4, 5, 6, 7, 8], 3);
    Solution::max_profit(vec![7, 1, 5, 3, 6, 4]);
}
