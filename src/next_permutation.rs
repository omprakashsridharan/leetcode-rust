fn reverse(nums: &mut Vec<i32>, s: i32) {
    let mut i = s;
    let mut j = (nums.len() - 1) as i32;
    while i < j {
        nums.swap(i as usize, j as usize);
        i += 1;
        j -= 1;
    }
}

pub fn next_permutation(nums: &mut Vec<i32>) {
    let mut i = (nums.len() - 2) as i32;
    while i >= 0 && nums.get((i + 1) as usize).unwrap() <= nums.get(i as usize).unwrap() {
        i -= 1;
    }
    if i >= 0 {
        let mut j = (nums.len() - 1) as i32;
        while nums[i as usize] >= nums[j as usize] {
            j -= 1;
        }
        nums.swap(i as usize, j as usize);
    }
    reverse(nums, i + 1)
}

#[cfg(test)]
mod test {
    use super::next_permutation;

    #[test]
    fn test_next_per() {
        let mut v = vec![1, 3, 2];
        next_permutation(&mut v);
        assert_eq!(v, vec![2, 1, 3]);
    }
}
