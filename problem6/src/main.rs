fn main() {
    let n = 100;
    println!(
        "{:?}",
        (1..n+1).sum::<isize>().pow(2)
        -
        (1..n+1).map(|x: isize| x.pow(2)).sum::<isize>()
    );
}
