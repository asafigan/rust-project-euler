fn main() {
    println!(
        "{}",
        (3..)
        .step_by(2)
        .filter(is_prime)
        .nth(10_001 - 2)
        .unwrap()
    );
}

fn is_prime(n: &isize) -> bool {
    !(3..)
    .step_by(2)
    .take_while(|&x| x <= n / x)
    .any(|x| n % x == 0 && *n != x)
}
