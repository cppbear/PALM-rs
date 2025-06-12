// Answer 0

#[test]
fn test_is_start_byte_0() {
    let b: u8 = 0;
    is_start_byte(b);
}

#[test]
fn test_is_start_byte_63() {
    let b: u8 = 63;
    is_start_byte(b);
}

#[test]
fn test_is_start_byte_64() {
    let b: u8 = 64;
    is_start_byte(b);
}

#[test]
fn test_is_start_byte_127() {
    let b: u8 = 127;
    is_start_byte(b);
}

#[test]
fn test_is_start_byte_128() {
    let b: u8 = 128;
    is_start_byte(b);
}

#[test]
fn test_is_start_byte_192() {
    let b: u8 = 192;
    is_start_byte(b);
}

#[test]
fn test_is_start_byte_193() {
    let b: u8 = 193;
    is_start_byte(b);
}

#[test]
fn test_is_start_byte_255() {
    let b: u8 = 255;
    is_start_byte(b);
}

#[test]
fn test_is_start_byte_128_to_191() {
    for b in 128..=191 {
        is_start_byte(b);
    }
}

#[test]
fn test_is_start_byte_192_to_255() {
    for b in 192..=255 {
        is_start_byte(b);
    }
}

