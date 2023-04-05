fn merge_sort(mut vec: Vec<i32>) -> Vec<i32> {
    if vec.len() <= 1 {
        return vec;
    }

    let mid = vec.len() / 2;
    let left_half = merge_sort(vec.drain(..mid).collect());
    let right_half = merge_sort(vec);

    let mut result = Vec::new();
    let mut left_iter = left_half.into_iter();
    let mut right_iter = right_half.into_iter();
    let mut left_peek = left_iter.next();
    let mut right_peek = right_iter.next();

    loop {
        match (left_peek, right_peek) {
            (Some(l), Some(r)) => {
                if l <= r {
                    result.push(l);
                    left_peek = left_iter.next();
                } else {
                    result.push(r);
                    right_peek = right_iter.next();
                }
            }
            (Some(l), None) => {
                result.push(l);
                result.extend(left_iter);
                break;
            }
            (None, Some(r)) => {
                result.push(r);
                result.extend(right_iter);
                break;
            }
            (None, None) => break,
        }
    }

    result
}
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let unsorted_vec: Vec<i32> = (0..10).map(|_| rng.gen_range(1..=100)).collect();
    println!("Unsorted vector: {:?}", unsorted_vec);
    let sorted_vec = merge_sort(unsorted_vec);
    println!("Sorted vector: {:?}", sorted_vec);
}
