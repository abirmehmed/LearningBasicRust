fn even_then_odd(numbers: Vec<i32>) -> Vec<i32> {
    let mut evens = Vec::new();
    let mut odds = Vec::new();
    for &number in &numbers {
        if number % 2 == 0 {
            evens.push(number);
        } else {
            odds.push(number);
        }
    }
    evens.extend(odds);
    evens
}
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let result = even_then_odd(numbers);
    println!("{:?}", result); // [2, 4, 1, 3, 5]
}
