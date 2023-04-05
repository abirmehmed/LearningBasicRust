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
use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let unsorted: Vec<i32> = (0..10).map(|_| rng.gen_range(0..100)).collect();
    println!("Unsorted: {:?}", unsorted);
    let sorted = bubble_sort(unsorted);
    println!("Sorted: {:?}", sorted);
}
