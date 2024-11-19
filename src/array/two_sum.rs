/*
Two Sum
Problem: Given an array of integers, return the indices of the two numbers such that they add up to a specific target.

Example:
Input: nums = [2, 7, 11, 15], target = 9
Output: [0, 1]

Time Complexity: O(n) using a hash map.
*/

use std::collections::HashMap;
use std::collections::HashSet;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&index) = map.get(&complement) {
            return vec![index as i32, i as i32];
        }

        map.insert(num, i);
    }
    vec![] //remove semicolon
}

pub fn longest_substring(s: &str) -> &str {
    let mut longest = "";
    let mut start = 0;
    let mut seen_chars = HashSet::new();

    for (end, c) in s.chars().enumerate() {
        // If we've seen the character before, move the start pointer to the right of the last occurrence
        println!("end: {} c: {}", end, c);
        while seen_chars.contains(&c) {
            // println!("char: {}", &s[start..].chars().next().unwrap());
            seen_chars.remove(&s[start..].chars().next().unwrap());
            start += s[start..].chars().next().unwrap().len_utf8();
        }

        // println!("len: {}", &s[start..].chars().next().unwrap().len_utf8());
        // Add the current character to the set of seen characters
        seen_chars.insert(c);
        println!("what's : {:?}", seen_chars);
        // Update the longest substring if the current one is longer
        if end - start + 1 > longest.len() {
            longest = &s[start..=end];
        }
    }

    longest
}

pub fn longest_substring_mixed_case(s: &str) -> String {
    let mut start = 0;
    let mut max_len = 0;
    let mut max_start = 0;
    let mut char_index_map = HashMap::new();

    for (end, c) in s.chars().enumerate() {
        if let Some(&i) = char_index_map.get(&c) {
            start = i + 1;
        }
        char_index_map.insert(c, end);

        if end - start + 1 > max_len {
            max_len = end - start + 1;
            max_start = start;
        }
        println!("start: {} end: {}", start, end);
        println!("char: {:?}", char_index_map);
    }

    s[max_start..max_start + max_len].to_string()
}

pub fn longest_substring_repeated_char(s: &str) -> &str {
    if s.is_empty() {
        return ""; // Edge case: empty string should return empty substring
    }

    let mut char_map = HashMap::new(); // Keeps track of the most recent index of each character
    let mut start = 0; // Left side of the sliding window
    let mut longest = ""; // To store the longest substring

    for (end, c) in s.chars().enumerate() {
        // If we encounter a character that has been seen before and is within the current window
        if let Some(&prev_index) = char_map.get(&c) {
            // Move the start of the window to the right of the last occurrence of the current character
            start = std::cmp::max(start, prev_index + 1);
        }

        // Update or add the current character's most recent index
        char_map.insert(c, end);

        // If the current window is larger than the previous longest substring, update it
        if end - start + 1 > longest.len() {
            longest = &s[start..=end];
        }
        println!("the substring: {}", longest)
    }

    longest
}
