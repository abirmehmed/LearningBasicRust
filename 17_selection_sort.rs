use rand::Rng;

fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_index = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut arr: Vec<i32> = (0..10).map(|_| rng.gen_range(0..100)).collect();
    println!("Before: {:?}", arr);

    selection_sort(&mut arr);
    println!("After: {:?}", arr);
}
