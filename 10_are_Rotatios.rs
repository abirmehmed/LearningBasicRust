fn are_rotations(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false;
    }
    let mut string1_chars = string1.chars().collect::<Vec<char>>();
    let string2_chars = string2.chars().collect::<Vec<char>>();
    for _ in 0..string1.len() {
        if string1_chars == string2_chars {
            return true;
        }
        let first_char = string1_chars.remove(0);
        string1_chars.push(first_char);
    }
    false
}
fn main() {
    let string1 = "abc";
    let string2 = "bca";
    let result = are_rotations(string1, string2);
    println!("Are '{}' and '{}' rotations of each other? {}", string1, string2, result);
}
