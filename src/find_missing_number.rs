pub fn run() {
    println!("========");
    println!("Missing number");
    println!("{:?}", missing_number(vec![3, 0, 1]));
    println!("========");
}
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let expected_sum = ((n * (n + 1)) / 2) as i32;
    let actual_sum: i32 = nums.iter().sum();
    expected_sum - actual_sum
}
