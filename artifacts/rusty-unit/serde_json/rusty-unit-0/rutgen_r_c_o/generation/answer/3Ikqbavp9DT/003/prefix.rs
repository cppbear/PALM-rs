// Answer 0

#[test]
fn test_fmt_pos_int_0() {
    let number = Number { n: N::PosInt(0) };
    let mut buffer = String::new();
    let _ = number.fmt(&mut buffer);
}

#[test]
fn test_fmt_pos_int_1() {
    let number = Number { n: N::PosInt(1) };
    let mut buffer = String::new();
    let _ = number.fmt(&mut buffer);
}

#[test]
fn test_fmt_pos_int_18446744073709551615() {
    let number = Number { n: N::PosInt(18446744073709551615) };
    let mut buffer = String::new();
    let _ = number.fmt(&mut buffer);
}

