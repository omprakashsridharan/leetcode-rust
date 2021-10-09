pub fn run() {
    println!("========");
    println!("Disappeared subarray");
    println!("{:?}", find_disappeared_numbers(vec![4, 2, 2, 1]));
    println!("========");
}

pub fn find_disappeared_numbers(n: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut zeros: Vec<i32> = vec![0; n.len()];
    let nums = n.clone();
    for i in 0..nums.len() {
        let num = nums[i];
        let index = (num - 1) as usize;
        zeros[index] = 1;
    }
    for i in 0..nums.len() {
        if zeros[i] == 0 {
            result.push((i + 1) as i32);
        }
    }
    result
}
