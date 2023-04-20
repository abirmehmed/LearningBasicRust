fn count_consonants (s : &str) -> usize {
 let vowels = [ 'a','e','i','o','u'];
 s.chars()
   .filter(|c| c.is_alphabetic() && !vowels.contains (&c.to_ascii_lowecase())).count()
   }
 fn main() {
  let s = "Hello World!";
  let num_consonants = count_consonants(s);
  println!("The number of consonants in '{}' is {}"'s' num_consonants);
}
