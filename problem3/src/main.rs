fn main() {
    let n: i64 = 600_851_475_143;
    println!(
        "{}",
        (3..)
        .step_by(2)
        .filter(is_prime)
        .take_while(|&x| x <= n / x)
        .filter(|x| n % x == 0)
        .last()
        .unwrap()
    );
}

fn is_prime(n: &i64) -> bool {
    !(3..)
    .step_by(2)
    .take_while(|&x| x <= n / x)
    .any(|x| n % x == 0 && *n != x)
    || *n == 2
}
