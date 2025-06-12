// Answer 0

#[test]
fn test_format_u8_bound_n_100() {
    let mut out = [0u8; 3];
    let n = 100;
    format_u8(n, &mut out);
}

#[test]
fn test_format_u8_above_bound_n_101() {
    let mut out = [0u8; 3];
    let n = 101;
    format_u8(n, &mut out);
}

#[test]
fn test_format_u8_above_bound_n_150() {
    let mut out = [0u8; 3];
    let n = 150;
    format_u8(n, &mut out);
}

#[test]
fn test_format_u8_above_bound_n_200() {
    let mut out = [0u8; 3];
    let n = 200;
    format_u8(n, &mut out);
}

#[test]
fn test_format_u8_above_bound_n_255() {
    let mut out = [0u8; 3];
    let n = 255;
    format_u8(n, &mut out);
}

