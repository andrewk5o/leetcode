mod merge_sorted_array;
mod remove_duplicates_from_sorted_array;
mod remove_element;

fn main() {
    merge_sorted_array::Solution::merge(
        &mut vec![1, 2, 3, 0, 0, 0],
        3,
        &mut vec![2, 5, 6],
        3,
    );
    remove_element::Solution::remove_element(&mut vec![3, 2, 2, 3], 3);
    remove_duplicates_from_sorted_array::Solution::remove_duplicates(
        &mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
    );
}
