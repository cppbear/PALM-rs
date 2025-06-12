fn is_valid(b: u8) -> bool {
    b >= 32 && b != 127 || b == b'\t'
}