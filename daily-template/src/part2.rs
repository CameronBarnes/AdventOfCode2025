
#[tracing::instrument]
pub fn process(_input: &str) -> impl IntoString {
    todo!("{{project-name}} - part 2")
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
