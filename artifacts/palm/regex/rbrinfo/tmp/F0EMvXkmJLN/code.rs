fn is_start_byte(b: u8) -> bool {
    b & 0b11_000000 != 0b1_0000000
}