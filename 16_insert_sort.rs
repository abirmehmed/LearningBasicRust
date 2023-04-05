fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}
fn main() {
    let mut arr = [5, 2, 4, 6, 1, 3];
    println!("Unsorted: {:?}", arr);

    insertion_sort(&mut arr);
    println!("Sorted: {:?}", arr);
}
