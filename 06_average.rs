fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let result = average(numbers);
    println!("The average is: {}", result);
}

fn average(numbers: Vec<i32>) -> f32 {
    let sum: i32 = numbers.iter().sum();
    sum as f32 / numbers.len() as f32
}
