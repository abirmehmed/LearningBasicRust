fn permute(data: &[char], i: usize, length: usize) {
    if length == i {
        println!("{}", data.iter().collect::<String>());
        return;
    }
    for j in i..length {
        let mut data = data.to_vec();
        data.swap(i, j);
        permute(&data, i + 1, length);
    }
}

fn main() {
    let string = "abc";
    let data: Vec<char> = string.chars().collect();
    permute(&data, 0, data.len());
}
