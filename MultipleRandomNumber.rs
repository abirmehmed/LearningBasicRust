use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let n: u32 = rng.gen_range(0..=100);
        println!("{}", n);
    }
}
