use regex::Regex;
use std::collections::HashSet;
pub fn check_char(a: char) {
    if a == 'a' || a == 'A' {
        println!("char: vowel");
    } else {
        println!("char: consonant");
    }
}

pub fn check_char1(c: char) {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("char1: vowel"),
        'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' | 'n' | 'p' | 'q' | 'r' | 's'
        | 't' | 'v' | 'w' | 'x' | 'y' | 'z' => println!("char1: consonant"),
        _ => println!("char1: Not a letter"),
    }
}

pub fn check_char2(ch: char) {
    let c = ch.to_ascii_lowercase();

    if "aeiou".contains(c) {
        println!("Char2: Vowel")
    } else if c.is_alphabetic() {
        println!("char2: consonant")
    } else {
        println!("char2: Not a letter")
    }
}
pub fn check_char3(ch: char) {
    let vowel_re = Regex::new(r"(?i)[aeiou]").unwrap();

    if vowel_re.is_match(&ch.to_string()) {
        println!("char3: vowel");
    } else if ch.is_alphabetic() {
        println!("char3: consonant");
    }
}

pub fn check_char_with_array(ch: char) {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    if vowels.contains(&ch.to_ascii_lowercase()) {
        println!("array method: vowel");
    } else if ch.is_alphabetic() {
        println!("array method: vowel");
    }
}

pub fn check_char_hashset(ch: char) {
    let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();
    if vowels.contains(&ch.to_ascii_lowercase()) {
        println!("array method: vowel");
    } else if ch.is_alphabetic() {
        println!("array method: vowel");
    }
}
