pub mod contains_duplicate;
pub mod excel_sheet_column_title;
pub mod find_all_duplicates;
pub mod find_disappeared_in_array;
pub mod find_missing_number;
pub mod find_unique_binary_strings;
pub mod maximum_subarray;
pub mod next_permutation;
pub mod sqrt;
pub mod two_sum;
pub mod two_sum_sorted;
fn main() {
    println!("Leetcode");
    two_sum::run();
    two_sum_sorted::run();
    excel_sheet_column_title::run();
    contains_duplicate::run();
    maximum_subarray::run();
    find_all_duplicates::run();
    find_disappeared_in_array::run();
    find_unique_binary_strings::run();
    find_missing_number::run();
    sqrt::run();
    let mut v = vec![3, 2, 1];
    next_permutation::next_permutation(&mut v);
    println!("Vec {:?}", v);
}
