mod merge_sorted_array;
mod remove_element;
mod remove_duplicates_from_sorted_array {
    pub mod i;
    pub mod ii;
}
mod majority_element;
mod rotate_array;
mod best_time_to_buy_and_sell_stock {
    pub mod i;
    pub mod ii;
}
mod jump_game {
    pub mod i;
    pub mod ii;
}

fn main() {
    merge_sorted_array::merge(
        &mut vec![1, 2, 3, 0, 0, 0],
        3,
        &mut vec![2, 5, 6],
        3,
    );
    remove_element::remove_element(&mut vec![3, 2, 2, 3], 3);
    remove_duplicates_from_sorted_array::i::remove_duplicates(&mut vec![
        0, 0, 1, 1, 1, 2, 2, 3, 3, 4,
    ]);
    remove_duplicates_from_sorted_array::ii::remove_duplicates(&mut vec![
        0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 4,
    ]);
    majority_element::majority_element(vec![
        2, 2, 2, 1, 2, 1, 2, 2, 1, 2, 2, 3, 3, 4,
    ]);
    rotate_array::rotate(&mut vec![1, 2, 3, 4, 5, 6, 7, 8], 3);
    best_time_to_buy_and_sell_stock::i::max_profit(vec![7, 1, 5, 3, 6, 4]);
    best_time_to_buy_and_sell_stock::ii::max_profit(vec![
        7, 1, 3, 5, 7, 6, 10, 2, 4,
    ]);
    jump_game::i::can_jump(vec![2, 3, 1, 1, 4]);
    jump_game::ii::jump(vec![2, 3, 1, 1, 4]);
}
