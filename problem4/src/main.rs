use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!(
        "{}",
        ((100 * 100)..(999 * 999))
        .filter(is_palindrome)
        .filter(is_product_of_two_three_digit_numbers)
        .rev()
        .next()
        .unwrap()
    );
    println!(
        "{}",
        start.elapsed().as_nanos()
    )
}

#[inline]
fn is_palindrome(n: &i32) -> bool {
    let x = n.abs() as usize;
    let mut digits = Digits::new(x);
    while let (Some(a), Some(b)) = (digits.next(), digits.next_back()) {
        if a != b {
            return false;
        }
    }
    true
}

#[inline]
fn is_product_of_two_three_digit_numbers(n: &i32) -> bool {
    (100..999)
    .rev()
    .filter(|x| n % x == 0)
    .map(|x| n / x)
    .any(|x| x >= 100 && x <= 999)
}

struct Digits {
    n: usize,
    divisor: usize,
}

impl Digits {
    #[inline]
    fn new(n: usize) -> Self {
        let mut divisor = 1;
        while n >= divisor * 10 {
            divisor *= 10;
        }

        Digits {
            n: n,
            divisor: divisor,
        }
    }
}

impl Iterator for Digits {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = Some(self.n / self.divisor);
            self.n %= self.divisor;
            self.divisor /= 10;
            v
        }
    }
}

impl DoubleEndedIterator for Digits {
    #[inline]
    fn next_back(&mut self) -> Option<usize> {
        if self.divisor == 0 {
            None
        } else {
            let v = Some(self.n % 10);
            self.n /= 10;
            self.divisor /= 10;
            v
        }
    }
}
