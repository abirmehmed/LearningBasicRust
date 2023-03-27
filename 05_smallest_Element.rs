fn smallest_element(arr: Vec<i32>) -> i32 {
    let mut smallest = arr[0];
    for &item in arr.iter() {
        if item < smallest {
            smallest = item;
        }
    }
    smallest
}
fn main() {
    let numbers = vec![5, 3, 8, 1, 4];
    let smallest = smallest_element(numbers);
    println!("The smallest element is: {}", smallest);
}
