pub mod contains_duplicate;
pub mod excel_sheet_column_title;
pub mod two_sum;
pub mod two_sum_sorted;

fn main() {
    println!("Leetcode");
    two_sum::run();
    two_sum_sorted::run();
    excel_sheet_column_title::run();
    contains_duplicate::run();
}
