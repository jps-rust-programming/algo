/*
Two Sum
Problem: Given an array of integers, return the indices of the two numbers such that they add up to a specific target.

Example:
Input: nums = [2, 7, 11, 15], target = 9
Output: [0, 1]

Time Complexity: O(n) using a hash map.
*/

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for () in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&index) = map.get(&complement) {
            return vec![index as i32, i as i32];
        }

        map.insert(num, i);
    }
    vec![];
}

pub fn longest_sub_string() {}
