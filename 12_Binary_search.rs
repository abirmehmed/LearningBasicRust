fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    None
}
fn main() {
    let arr = [1, 3, 4, 5, 7, 8, 9];
    let target = 5;

    match binary_search(&arr, target) {
        Some(index) => println!("{} was found at index {}", target, index),
        None => println!("{} was not found in the array", target),
    }
}
