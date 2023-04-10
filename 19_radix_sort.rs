fn radix_sort(arr: &mut [u32]) {
    let max = *arr.iter().max().unwrap();
    let mut exp = 1;
    let mut buffer = vec![0; arr.len()];

    while max / exp > 0 {
        let mut buckets = [0; 10];

        for &num in arr.iter() {
            buckets[(num / exp) as usize % 10] += 1;
        }

        for i in 1..10 {
            buckets[i] += buckets[i - 1];
        }

        for &num in arr.iter().rev() {
            buffer[buckets[(num / exp) as usize % 10] as usize - 1] = num;
            buckets[(num / exp) as usize % 10] -= 1;
        }

        arr.copy_from_slice(&buffer);
        exp *= 10;
    }
}
fn main() {
    let mut arr = [170, 45, 75, 90, 802, 24, 2, 66];
    println!("Original array: {:?}", arr);

    radix_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
