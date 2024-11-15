pub fn rotate_array(nums: &mut Vec<i32>, k: usize) {}

fn rotate(nums: &mut Vec<i32>, k: usize) {
    let n = nums.len();
    let k = k % n; // Handle k > n
    nums.reverse();
    nums[0..k].reverse();
    nums[k..].reverse();
}
