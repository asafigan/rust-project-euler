fn main() {
    println!(
        "{}",
        Fibonacci::new(1,2)
        .filter(|x| x % 2 == 0)
        .take_while(|&x| x < 4_000_000)
        .fold(0, |x, acc| x + acc)
    );
}

struct Fibonacci {
    a: i32,
    b: i32,
}

impl Fibonacci {
    fn new(a: i32, b: i32) -> Fibonacci {
        Fibonacci { a: a, b: b }
    }
}

impl Iterator for Fibonacci {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.a;
        self.a = self.b;
        self.b += a;

        Some(a)
    }
}
