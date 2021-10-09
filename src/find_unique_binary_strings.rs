use std::collections::HashSet;

pub fn run() {
    println!("========");
    println!("Find unique binary strings");
    println!(
        "{:?}",
        find_different_binary_string(vec!["010".into(), "001".into(), "000".into()])
    );
    println!("========");
}

pub fn find_different_binary_string(nums: Vec<String>) -> String {
    let n = nums.len();
    let mut set: HashSet<isize> = HashSet::new();
    for num in nums {
        set.insert(isize::from_str_radix(&num, 2).unwrap());
    }
    for i in 0..((2 as i32).pow(n as u32)) {
        let present_option = set.get(&(i as isize));
        match present_option {
            None => {
                return format!("{number:0>width$b}", number = i, width = n);
            }
            _ => {}
        }
    }
    String::new()
}
