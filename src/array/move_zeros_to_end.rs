/*
Move Zeros to the End
Problem: Given an array, move all the zeros to the end while maintaining the relative order of the non-zero elements.

Example:
Input: arr = [0, 1, 0, 3, 12]
Output: [1, 3, 12, 0, 0]

Time Complexity: O(n)
*/
pub fn move_zeros_to_end(nums: &mut Vec<i32>) {
    let mut index = 0;

    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(i, index);
            index += 1;
        }
    }
}
