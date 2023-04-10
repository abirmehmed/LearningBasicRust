fn counting_sort(arr: &mut [u32], maxval: u32) {
    let mut occurences: Vec<u32> = vec![0; maxval as usize + 1];

    for &data in arr.iter() {
        occurences[data as usize] += 1;
    }

    let mut i = 0;
    for (data, &number) in occurences.iter().enumerate() {
        for _ in 0..number {
            arr[i] = data as u32;
            i += 1;
        }
    }
}
fn main() {
    let mut arr: [u32; 8] = [3, 6, 4, 1, 3, 4, 1, 4];
    let maxval = *arr.iter().max().unwrap();
    counting_sort(&mut arr, maxval);
    println!("Sorted array: {:?}", arr);
}
