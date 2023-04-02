fn are_palindromes(string1: &str, string2: &str) -> bool {
    string1.chars().rev().collect::<String>() == string1 && string2.chars().rev().collect::<String>() == string2
}
fn main() {
    let string1 = "racecar";
    let string2 = "level";
    let result = are_palindromes(string1, string2);
    println!("Are '{}' and '{}' palindromes? {}", string1, string2, result);
}
