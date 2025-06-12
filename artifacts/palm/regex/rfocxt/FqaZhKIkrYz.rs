use std::char;
const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO: u8 = 0b1100_0000;
const TAG_THREE: u8 = 0b1110_0000;
const TAG_FOUR: u8 = 0b1111_0000;
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
fn is_start_byte(b: u8) -> bool {
    b & 0b11_000000 != 0b1_0000000
}
#[inline]
pub fn decode_utf8(src: &[u8]) -> Option<(char, usize)> {
    let b0 = match src.get(0) {
        None => return None,
        Some(&b) if b <= 0x7F => return Some((b as char, 1)),
        Some(&b) => b,
    };
    match b0 {
        0b110_00000..=0b110_11111 => {
            if src.len() < 2 {
                return None;
            }
            let b1 = src[1];
            if 0b11_000000 & b1 != TAG_CONT {
                return None;
            }
            let cp = ((b0 & !TAG_TWO) as u32) << 6 | ((b1 & !TAG_CONT) as u32);
            match cp {
                0x80..=0x7FF => char::from_u32(cp).map(|cp| (cp, 2)),
                _ => None,
            }
        }
        0b1110_0000..=0b1110_1111 => {
            if src.len() < 3 {
                return None;
            }
            let (b1, b2) = (src[1], src[2]);
            if 0b11_000000 & b1 != TAG_CONT {
                return None;
            }
            if 0b11_000000 & b2 != TAG_CONT {
                return None;
            }
            let cp = ((b0 & !TAG_THREE) as u32) << 12 | ((b1 & !TAG_CONT) as u32) << 6
                | ((b2 & !TAG_CONT) as u32);
            match cp {
                0x800..=0xFFFF => char::from_u32(cp).map(|cp| (cp, 3)),
                _ => None,
            }
        }
        0b11110_000..=0b11110_111 => {
            if src.len() < 4 {
                return None;
            }
            let (b1, b2, b3) = (src[1], src[2], src[3]);
            if 0b11_000000 & b1 != TAG_CONT {
                return None;
            }
            if 0b11_000000 & b2 != TAG_CONT {
                return None;
            }
            if 0b11_000000 & b3 != TAG_CONT {
                return None;
            }
            let cp = ((b0 & !TAG_FOUR) as u32) << 18 | ((b1 & !TAG_CONT) as u32) << 12
                | ((b2 & !TAG_CONT) as u32) << 6 | ((b3 & !TAG_CONT) as u32);
            match cp {
                0x10000..=0x10FFFF => char::from_u32(cp).map(|cp| (cp, 4)),
                _ => None,
            }
        }
        _ => None,
    }
}
