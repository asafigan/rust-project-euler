fn main() {
    println!(
        "{}",
        (1..)
        .find(|x| (2..20).all(|y| x % y == 0))
        .unwrap()
    );
}
