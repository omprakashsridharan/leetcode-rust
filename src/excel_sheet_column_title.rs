pub fn run() {
    println!("========");
    println!("Excel sheet column title");
    println!("{:?}", convert_to_title(703));
    println!("========");
}

pub fn convert_to_title(column_number: i32) -> String {
    let mut result = String::new();
    let mut quotient = (column_number - 1) / 26;
    let remainder = (column_number - 1) % 26;
    loop {
        if quotient > 0 {
            result.push(((quotient - 1) % 26 + 65) as u8 as char);
            quotient = (quotient - 1) / 26;
        } else {
            result = result.chars().rev().collect::<String>();
            result.push((remainder + 65) as u8 as char);
            break;
        }
    }
    return result;
}
