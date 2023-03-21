use std::collections::HashMap;

fn first_non_repeated_char(s: &str) -> Option<char> {
    let mut char_counts = HashMap::new();
    for c in s.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    s.chars().find(|c| char_counts[c] == 1)
}
fn main() {
    let s = "abacabad";
    if let Some(c) = first_non_repeated_char(s) {
        println!("The first non-repeated character is: {}", c);
    } else {
        println!("There are no non-repeated characters.");
    }
}
