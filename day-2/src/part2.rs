use itertools::Itertools;

fn check_valid_id(id: u64) -> bool {
    let str = id.to_string();
    let content = str.as_bytes();
    if content.len().is_multiple_of(2) {
        let max = content.len() / 2;
        (1..=max).any(|size| {
            let mut chunks = content.chunks_exact(size);
            chunks.remainder().is_empty() && chunks.all_equal()
        })
    } else {
        false
    }
}

#[must_use]
pub fn process(_input: &str) -> String {
    todo!("day-2 - part 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        todo!("Havent built test yet");
        let input = "";
        assert_eq!("", process(input));
    }
}
