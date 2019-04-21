fn main() {
    println!(
        "{:?}",
        (3..=2_000_000_u64).step_by(2).filter(is_prime).sum::<u64>() + 2_u64
    );
}

fn is_prime(n: &u64) -> bool {
    !(3..)
    .step_by(2)
    .take_while(|&x| x <= n / x)
    .any(|x| n % x == 0 && *n != x)
}
