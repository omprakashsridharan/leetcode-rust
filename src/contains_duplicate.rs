use std::collections::HashMap;

pub fn run() {
    println!("========");
    println!("Contains duplicate");
    println!("{:?}", contains_duplicate(vec![1, 2, 3, 2]));
    println!("========");
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map: HashMap<i32, bool> = HashMap::new();
    for num in nums {
        if let Some(_) = map.get(&num) {
            return true;
        } else {
            map.insert(num, true);
        }
    }
    return false;
}
