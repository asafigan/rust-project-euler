fn main() {
    println!(
        "{}",
        (1..10_000)
        .filter(|&a| {
            let b = sum_of_proper_divisors(a);
            a == sum_of_proper_divisors(b) && a != b
        })
        .sum::<u32>()
    );
}

fn sum_of_proper_divisors(n: u32) -> u32 {
    (1..n)
    .filter(|x| n % x == 0)
    .sum()
}
