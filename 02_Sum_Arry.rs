fn sum_array(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in arr {
        sum += num;
    }
    sum
}
fn main() {
    let my_array = vec![1, 2, 3, 4];
    let result = sum_array(&my_array);
    println!("The sum is: {}", result);
}
