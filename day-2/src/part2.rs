use itertools::Itertools;

fn check_valid_id(id: u64) -> bool {
    let str = id.to_string();
    let content = str.as_bytes();
    let max = content.len() / 2;
    (1..=max).any(|size| {
        let mut chunks = content.chunks_exact(size);
        chunks.remainder().is_empty() && chunks.all_equal()
    })
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn process(input: &str) -> String {
    input
        .trim_end()
        .split(',')
        .flat_map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
        })
        .filter(|num| check_valid_id(*num))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("4174379265", process(input));
    }

    #[test]
    fn test_process() {
        let input = include_str!("../input.txt");
        assert_eq!("45814076230", process(input));
    }
}
