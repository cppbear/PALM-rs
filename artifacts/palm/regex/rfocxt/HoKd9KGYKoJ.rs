use std::char;
const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO: u8 = 0b1100_0000;
const TAG_THREE: u8 = 0b1110_0000;
const TAG_FOUR: u8 = 0b1111_0000;
pub fn next_utf8(text: &[u8], i: usize) -> usize {
    let b = match text.get(i) {
        None => return i + 1,
        Some(&b) => b,
    };
    let inc = if b <= 0x7F {
        1
    } else if b <= 0b110_11111 {
        2
    } else if b <= 0b1110_1111 {
        3
    } else {
        4
    };
    i + inc
}
