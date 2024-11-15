mod prime;
mod vowel_consonant; // mod(keyword) filename
fn main() {
    println!("Hello, world!");
    let b = find_max(4, 3);

    println!("The max value: {}", b);
    vowel_consonant::check_char('a');
    vowel_consonant::check_char2('r');
    vowel_consonant::check_char1('c');
    vowel_consonant::check_char3('c');
    vowel_consonant::check_char_with_array('d');
    vowel_consonant::check_char_hashset('f');
    prime::prime::prime_number();
    //  folder_name::filen_ame::function_name
}

fn find_max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
