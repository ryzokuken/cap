pub fn non_ascii_whitespace_test(ch: char) -> bool {
    ch.is_whitespace() && !ch.is_ascii_whitespace()
}
