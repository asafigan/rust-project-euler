
fn main() {
    println!("{}", Range{value: 1, to: 10}.filter(is_multiple).fold(0, sum));
}

fn sum(acc: i32, x: i32) -> i32 {
    acc + x
}

fn is_multiple(x: &i32) -> bool {
    x % 3 == 0 || x % 5 == 0
}

pub enum Option<T> {
    None,
    Some(T),
}

trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    #[inline]
    fn filter<P>(self, predicate: P) -> Filter<Self, P> where
        Self: Sized, P: Fn(&Self::Item) -> bool,
    {
        Filter {iter: self, predicate }
    }

    #[inline]
    fn fold<B, F>(&mut self, init: B, f: F) -> B where
        Self: Sized, F: Fn(B, Self::Item) -> B
    {
        let mut accum = init;
        while let Option::Some(x) = self.next() {
            accum = f(accum, x);
        }
        accum
    }
}

struct Filter<I, P> {
    iter: I,
    predicate: P,
}

impl<I: Iterator, P> Iterator for Filter<I, P> where P: Fn(&I::Item) -> bool {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        while let Option::Some(x) = self.iter.next() {
            if (self.predicate)(&x) {
                return Option::Some(x);
            }
        }
        Option::None
    }
}

struct Range {
    to: i32,
    value: i32,
}

impl Iterator for Range {
    type Item = i32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.value;
        self.value += 1;
        if value <= self.to {
            return Option::Some(value);
        }
        Option::None
    }
}
