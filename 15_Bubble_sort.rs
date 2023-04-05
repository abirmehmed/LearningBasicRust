fn bubble_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
    nums
}
fn main() {
    let unsorted = vec![4, 2, 3, 1];
    let sorted = bubble_sort(unsorted);
    println!("Sorted: {:?}", sorted);
}
