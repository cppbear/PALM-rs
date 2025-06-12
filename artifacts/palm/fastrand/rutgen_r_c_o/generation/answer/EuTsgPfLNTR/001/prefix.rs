// Answer 0

#[test]
fn test_choose_multiple_with_u8() {
    let source: Vec<u8> = (0..255).collect();
    let amount = 10;
    choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_i8() {
    let source: Vec<i8> = (-128..127).collect();
    let amount = 5;
    choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_u16() {
    let source: Vec<u16> = (0..65535).collect();
    let amount = 20;
    choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_i16() {
    let source: Vec<i16> = (-32768..32767).collect();
    let amount = 15;
    choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_u32() {
    let source: Vec<u32> = (0..4294967295).collect();
    let amount = 30;
    choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_i32() {
    let source: Vec<i32> = (-2147483648..2147483647).collect();
    let amount = 25;
    choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_u64() {
    let source: Vec<u64> = (0..18446744073709551615).collect();
    let amount = 2;
    choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_i64() {
    let source: Vec<i64> = (-9223372036854775808..9223372036854775807).collect();
    let amount = 3;
    choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_u128() {
    let source: Vec<u128> = (0..340282366920938463463374607431768211455).collect();
    let amount = 1;
    choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_i128() {
    let source: Vec<i128> = (-170141183460469231731687303715884105728..170141183460469231731687303715884105727).collect();
    let amount = 4;
    choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_usize() {
    let source: Vec<usize> = (0..18446744073709551615).collect();
    let amount = 7;
    choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_isize() {
    let source: Vec<isize> = (-9223372036854775808..9223372036854775807).collect();
    let amount = 6;
    choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_char() {
    let source: Vec<char> = ('\u{0000}'..='\u{FFFF}').collect();
    let amount = 50;
    choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_zero_amount() {
    let source: Vec<u8> = (0..255).collect();
    let amount = 0;
    choose_multiple(source, amount);
}

#[test]
#[should_panic]
fn test_choose_multiple_with_empty_source() {
    let source: Vec<u8> = Vec::new();
    let amount = 1;
    choose_multiple(source, amount);
}

