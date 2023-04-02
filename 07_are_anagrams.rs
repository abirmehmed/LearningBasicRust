use std::collections::HashMap;

fn are_anagrams(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut char_counts = HashMap::new();

    for c in s1.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        let count = char_counts.entry(c).or_insert(0);
        *count -= 1;
        if *count < 0 {
            return false;
        }
    }

    true
}
fn main() {
    let s1 = "listen";
    let s2 = "silent";
    let result = are_anagrams(s1, s2);
    println!("Are '{}' and '{}' anagrams? {}", s1, s2, result);
}
