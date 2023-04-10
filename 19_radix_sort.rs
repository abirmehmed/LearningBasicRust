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
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut arr: [u32; 10] = [0; 10];

    for i in 0..10 {
        arr[i] = rng.gen_range(0..100);
    }

    println!("Original array: {:?}", arr);

    radix_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
