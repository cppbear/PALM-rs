pub fn decode_last_utf8(src: &[u8]) -> Option<(char, usize)> {
    if src.is_empty() {
        return None;
    }
    let mut start = src.len() - 1;
    if src[start] <= 0x7F {
        return Some((src[start] as char, 1));
    }
    while start > src.len().saturating_sub(4) {
        start -= 1;
        if is_start_byte(src[start]) {
            break;
        }
    }
    match decode_utf8(&src[start..]) {
        None => None,
        Some((_, n)) if n < src.len() - start => None,
        Some((cp, n)) => Some((cp, n)),
    }
}