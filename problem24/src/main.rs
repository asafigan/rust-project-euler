fn main() {
    println!("{}", nth_lexicographic_permutation(1_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_1() {
        assert_eq!(nth_lexicographic_permutation(1), 123456789);
    }

    #[test]
    fn case_3_628_800() {
        assert_eq!(nth_lexicographic_permutation(3_628_800), 9876543210);
    }
}

fn nth_lexicographic_permutation(mut n: u64) -> u64 {
    n = n - 1;

    let mut options = vec![0,1,2,3,4,5,6,7,8,9];
    let mut digits = Vec::new();

    for number_of_digits in (1..=(options.len() as u64)).rev() {
        let combinations: u64 = (1..number_of_digits).product();
        for x in 1..=number_of_digits {
            if x * combinations > n {
                n = n - (x - 1) * combinations;
                digits.push(options.remove((x - 1) as usize));
                break;
            }
        }
    }

    return digits_to_u64(&digits[..])
}

fn digits_to_u64(digits: &[u64]) -> u64 {
    digits
    .iter()
    .fold(0, |number, digit| number * 10 + digit)
}
