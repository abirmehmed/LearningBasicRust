fn count_vowels(s: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    s.chars().filter(|c| vowels.contains(c)).count()
}
let s = "Hello, world!";
let n_vowels = count_vowels(s);
println!("The string '{}' contains {} vowels.", s, n_vowels);
