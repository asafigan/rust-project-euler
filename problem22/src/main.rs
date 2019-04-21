static DATA: &str = include_str!("names.txt");

fn main() {
    let mut names =
        DATA
        .trim()
        .split(",")
        .map(|s| &s[1..s.len() - 1])
        .collect::<Vec<_>>();

    names.sort_unstable();

    println!(
        "{}",
        names
        .iter()
        .enumerate()
        .map(|(index, name)|
            name
            .chars()
            .map(|c| letter_to_digit(c))
            .sum::<u32>() * ((index + 1) as u32)
        )
        .sum::<u32>()
    );
}

enum StaticOrDynamic {
    Static(&'static str),
    Dynamic(String)
}

fn test() -> Vec<StaticOrDynamic> {
    vec![
        StaticOrDynamic::Static(&"Static"),
        StaticOrDynamic::Dynamic(5.to_string()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn converts_letter_to_digit() {
        assert_eq!(letter_to_digit('A'), 1);
        assert_eq!(letter_to_digit('Z'), 26);
    }
}

fn letter_to_digit(c: char) -> u32 {
    c.to_digit(36).unwrap() - 9
}


