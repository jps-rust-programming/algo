// linear search O(n)
pub fn max(nums: &[i32]) -> Option<i32> {
    if nums.is_empty() {
        return None;
    }

    let mut max = nums[0];

    for &num in nums.iter().skip(1) {
        if num > max {
            max = num;
        }
    }
    return Some(max);
}


// pub fn 