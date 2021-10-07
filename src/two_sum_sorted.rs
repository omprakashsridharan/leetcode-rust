use std::collections::HashMap;

pub fn run() {
    println!("========");
    println!("Two sum sorted");
    println!("{:?}", two_sum_sorted(vec![2, 7, 11, 15], 9));
    println!("========");
}

fn two_sum_sorted(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (pos, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(val) = map.get(&complement) {
            if ((pos + 1) as i32) < *val + 1 {
                result.push((pos + 1) as i32);
                result.push(*val + 1);
            } else {
                result.push(*val + 1);
                result.push((pos + 1) as i32);
            }
            return result;
        }
        map.insert(*num, pos as i32);
    }
    return result;
}
