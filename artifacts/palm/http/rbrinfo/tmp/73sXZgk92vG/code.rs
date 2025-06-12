const fn is_visible_ascii(b: u8) -> bool {
    b >= 32 && b < 127 || b == b'\t'
}