use std::collections::HashSet;
/*
Longest Consecutive Sequence
Problem: Given an unsorted array, find the length of the longest consecutive elements sequence.

Example:
Input: arr = [100, 4, 200, 1, 3, 2]
Output: 4 (The longest consecutive sequence is [1, 2, 3, 4])

Time Complexity: O(n)
*/

pub fn consecutive_sequence(nums: Vec<i32>) -> i32 {
    let mut nums_set: HashSet<i32> = nums.into_iter().collect();
    let mut longest_streak = 0;

    for &num in nums_set.iter() {
        if !nums_set.contains(&(num - 1)) {
            let mut current_num = num;
            let mut current_streak = 1;
            while nums_set.contains(&(current_num + 1)) {
                current_num += 1;
                current_streak += 1;
            }

            longest_streak = longest_streak.max(current_streak);
        }
    }
    return 0;
}
