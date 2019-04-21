fn main() {
    time(|| {
        println!("{}", for_loop_solution().unwrap())
    });

    time(|| {
        println!("{}", iter_solution().unwrap())
    });
}

fn time<A>(x: A)
    where A: Fn()->()
{
    let start = std::time::Instant::now();
    x();
    println!("{}", start.elapsed().as_millis());
}

fn for_loop_solution() -> Option<usize> {
    for a in 1usize..=1000 {
        for b in 1usize..=1000 {
            for c in 1usize..=1000 {
                if a + b + c == 1000 && a.pow(2) + b.pow(2) == c.pow(2) {
                    return Some(a * b * c);
                }
            }
        }
    }
    None
}

fn iter_solution() -> Option<usize> {
    let range = 1usize..=1000;
    range.clone()
    .flat_map(|x|
        range.clone().zip(std::iter::repeat(x))
    )
    .flat_map(|x|
        range.clone().zip(std::iter::repeat(x))
    )
    .filter(|(c, (b, a))| a + b + c == 1000 && a.pow(2) + b.pow(2) == c.pow(2))
    .map(|(c, (b, a))| a * b * c)
    .nth(0)
}
