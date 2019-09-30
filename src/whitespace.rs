pub fn is_newline(ch: char, ecma_2019_string: bool) -> bool {
    let code = ch as u16;
    code == 10 || code == 13 || (!ecma_2019_string && (code == 0x2028 || code == 0x2029))
}
