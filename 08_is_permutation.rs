fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut chars1: Vec<char> = s1.chars().collect();
    let mut chars2: Vec<char> = s2.chars().collect();
    chars1.sort();
    chars2.sort();
    chars1 == chars2
}
fn main() {
    let result = is_permutation("abc", "bca");
    println!("{}", result); // prints true
}
