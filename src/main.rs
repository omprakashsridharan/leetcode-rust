use std::collections::HashMap;

mod two_sum;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32,i32> = HashMap::new();
    let mut result: Vec<i32> = vec![];
    for (pos,num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(val) = map.get(&complement) {
            result.push(pos as i32);
            result.push(*val);
            return result;
        }
        map.insert(*num, pos as i32);
    }
    return result;
}

fn main() {
    println!("{:?}",two_sum(vec![2,7,11,15],9))
}
