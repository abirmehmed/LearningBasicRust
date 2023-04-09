fn heap_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in (0..len / 2).rev() {
        heapify(arr, len, i);
    }
    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify(arr: &mut [i32], len: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    if left < len && arr[left] > arr[largest] {
        largest = left;
    }
    if right < len && arr[right] > arr[largest] {
        largest = right;
    }
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, len, largest);
    }
}
