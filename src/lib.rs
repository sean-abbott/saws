pub fn find_match(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    if content.contains(pattern) {
        writeln!(writer, "{}", content).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_a_match() {
        let mut result = Vec::new();
        find_match("lorem ipsum", "lorem", &mut result);
        assert_eq!(result, b"lorem ipsum\n");
    }
}
