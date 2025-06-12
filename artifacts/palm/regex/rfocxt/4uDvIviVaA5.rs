use std::char;
const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO: u8 = 0b1100_0000;
const TAG_THREE: u8 = 0b1110_0000;
const TAG_FOUR: u8 = 0b1111_0000;
fn is_start_byte(b: u8) -> bool {
    b & 0b11_000000 != 0b1_0000000
}
