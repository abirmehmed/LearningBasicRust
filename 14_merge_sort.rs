fn merge_sort(mut vec: Vec<i32>) -> Vec<i32> {
    if vec.len() <= 1 {
        return vec;
    }

    let mid = vec.len() / 2;
    let left = merge_sort(vec[..mid].to_vec());
    let right = merge_sort(vec[mid..].to_vec());

    let mut left_iter = left.into_iter();
    let mut right_iter = right.into_iter();
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            vec[i + j] = left_iter.next().unwrap();
            i += 1;
        } else {
            vec[i + j] = right_iter.next().unwrap();
            j += 1;
        }
    }

    while i < left.len() {
        vec[i + j] = left_iter.next().unwrap();
        
        i += 1;
    }

    while j < right.len() {
        vec[i + j] = right_iter.next().unwrap();
        j += 1;
    }

    vec
}
fn main() {
    let unsorted_list = vec![4, 2, 3, 1];
    let sorted_list = merge_sort(unsorted_list);
    println!("{:?}", sorted_list);
}
