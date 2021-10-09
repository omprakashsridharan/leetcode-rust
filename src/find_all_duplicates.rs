pub fn run() {
    println!("========");
    println!("Find all duplicates");
    println!("{:?}", find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]));
    println!("========");
}

pub fn find_duplicates(n: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut nums = n.clone();
    for i in 0..nums.len() {
        let num = nums[i];
        let index = (num.abs() - 1) as usize;
        if nums[index] < 0 {
            result.push(num.abs());
        }
        nums[index] *= -1;
    }
    result
}
