pub fn run() {
    println!("========");
    println!("Excel sheet column title");
    println!("{:?}", convert_to_title(2147483647));
    println!("========");
}

pub fn convert_to_title(column_number: i32) -> String {
    let mut result = String::new();
    let mut num = column_number;
    while num > 0 {
        result.push((65 + ((num - 1) % 26)) as u8 as char);
        num = (num - 1) / 26;
    }
    return result.chars().rev().collect::<String>();
}
