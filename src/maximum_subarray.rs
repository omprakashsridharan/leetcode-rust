pub fn run() {
    println!("========");
    println!("Maximum subarray");
    println!("{:?}", max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
    println!("========");
}

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    // return find_best_array(nums.clone(), 0, (nums.len() - 1) as i32);
    let mut curr_sum = nums[0];
    let mut max_sum = nums[0];
    for i in 1..nums.len() {
        curr_sum = nums[i].max(curr_sum + nums[i]);
        max_sum = curr_sum.max(max_sum);
    }
    return max_sum;
}

// Dynamic programming
fn _find_best_array(nums: Vec<i32>, left: i32, right: i32) -> i32 {
    if left > right {
        return i32::MIN;
    }
    let mid = (left + right) / 2;
    let mut curr: i32 = 0;
    let mut left_best_sum = 0;
    let mut right_best_sum = 0;
    for i in (left..=(mid - 1)).rev() {
        curr += nums[i as usize];
        left_best_sum = curr.max(left_best_sum);
    }
    curr = 0;
    for i in (mid + 1)..(right + 1) {
        curr += nums[i as usize];
        right_best_sum = curr.max(right_best_sum);
    }

    let best_combined_sum: i32 = nums[mid as usize] + left_best_sum + right_best_sum;
    let left_half = _find_best_array(nums.clone(), left, mid - 1);
    let right_half = _find_best_array(nums.clone(), mid + 1, right);
    return best_combined_sum.max(left_half.max(right_half));
}
