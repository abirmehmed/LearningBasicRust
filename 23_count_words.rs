fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}
fn main() {
    let my_string = "This is a test string";
    let word_count = count_words(my_string);
    println!("The number of words is: {}", word_count);
}
