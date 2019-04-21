const NUMBER: usize = 1_000_000_000;
fn main() {
    time(|| {
        println!("{}", (1..=NUMBER).filter(is_multiple).sum::<usize>());
    });

    time(|| {
        println!("{}", (1..=NUMBER).filter(|x| x % 3 == 0 || x % 5 == 0).sum::<usize>());
    });

    time(|| {
        let mut sum = 0;
        for x in 1..=NUMBER {
            if is_multiple(&x) {
                sum += x;
            }
        }
        println!("{}", {sum});
    });

    time(|| {
        let mut sum = 0;
        for x in 1..=NUMBER {
            if x % 3 == 0 || x % 5 == 0 {
                sum += x;
            }
        }
        println!("{}", {sum});
    });
}

#[inline]
fn is_multiple(x: &usize) -> bool {
    x % 3 == 0 || x % 5 == 0
}

fn time<A>(x: A)
    where A: Fn()->()
{
    let start = std::time::Instant::now();
    x();
    println!("{}", start.elapsed().as_millis());
}
