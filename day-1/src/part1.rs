use core::unreachable;

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn process(input: &str) -> String {
    let mut num: i32 = 50;
    let mut zeros = 0;
    input
        .lines()
        .map(|line| {
            let pair = line.split_at(1);
            (pair.0, pair.1.parse::<u16>().unwrap())
        })
        .for_each(|(direction, number)| {
            let number = i32::from(number);
            match direction {
                "L" => {
                    num = (num - number) % 100;
                    if num < 0 {
                        num += 100;
                    }
                }
                "R" => num = (num + number) % 100,
                _ => unreachable!(),
            }
            if num == 0 {
                zeros += 1;
            }
        });
    zeros.to_string()
}

#[cfg(test)]
mod tests {
    use core::assert_eq;

    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            "3",
            process(
                "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
            )
        );
    }

    #[test]
    fn test_process() {
        let input = include_str!("../input.txt");
        assert_eq!("1100", process(input));
    }
}
