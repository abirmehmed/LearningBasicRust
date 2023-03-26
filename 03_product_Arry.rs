fn product_array(arr: &[i32]) -> i32 {
    let mut product = 1;
    for &num in arr {
        product *= num;
    }
    product
}
fn main() {
    let my_array = vec![1, 2, 3, 4];
    let result = product_array(&my_array);
    println!("The product is: {}", result);
}
