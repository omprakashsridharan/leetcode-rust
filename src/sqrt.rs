pub fn run() {
    println!("========");
    println!("Sqrt(x)");
    println!("{:?}", my_sqrt(8));
    println!("========");
}

pub fn my_sqrt(x: i32) -> i32 {
    if x <= 1 {
        return x;
    }
    let mut left = 0;
    let mut right = x / 2;
    let mut mid = 0;
    let mut ans = 0;
    let mut temp: i128 = 0;
    while left <= right {
        mid = (left + (right - left) / 2);
        println!("mid {}", mid);
        temp = mid as i128 * mid as i128;
        if temp == x as i128 {
            return mid;
        }
        if temp < x as i128 {
            ans = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    ans
}
