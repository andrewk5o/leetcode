mod merge_sorted_array;

fn main() {
    merge_sorted_array::Solution::merge(&mut vec![1, 2, 3, 0, 0, 0], 3, &mut vec![2, 5, 6], 3);
}
