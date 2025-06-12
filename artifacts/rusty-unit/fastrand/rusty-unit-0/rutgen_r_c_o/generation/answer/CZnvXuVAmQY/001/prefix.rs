// Answer 0

#[test]
fn test_rng_u8() {
    let range: std::ops::Range<u8> = 0..255;
    let result = with_rng(|r| r.u8(range));
}

#[test]
fn test_rng_i8() {
    let range: std::ops::Range<i8> = -128..127;
    let result = with_rng(|r| r.i8(range));
}

#[test]
fn test_rng_u16() {
    let range: std::ops::Range<u16> = 0..65535;
    let result = with_rng(|r| r.u16(range));
}

#[test]
fn test_rng_i16() {
    let range: std::ops::Range<i16> = -32768..32767;
    let result = with_rng(|r| r.i16(range));
}

#[test]
fn test_rng_u32() {
    let range: std::ops::Range<u32> = 0..4294967295;
    let result = with_rng(|r| r.u32(range));
}

#[test]
fn test_rng_i32() {
    let range: std::ops::Range<i32> = -2147483648..2147483647;
    let result = with_rng(|r| r.i32(range));
}

#[test]
fn test_rng_u64() {
    let range: std::ops::Range<u64> = 0..18446744073709551615;
    let result = with_rng(|r| r.u64(range));
}

#[test]
fn test_rng_i64() {
    let range: std::ops::Range<i64> = -9223372036854775808..9223372036854775807;
    let result = with_rng(|r| r.i64(range));
}

#[test]
fn test_rng_u128() {
    let range: std::ops::Range<u128> = 0..340282366920938463463374607431768211455;
    let result = with_rng(|r| r.u128(range));
}

#[test]
fn test_rng_i128() {
    let range: std::ops::Range<i128> = -170141183460469231731687303715884105728..170141183460469231731687303715884105727;
    let result = with_rng(|r| r.i128(range));
}

#[test]
fn test_rng_usize() {
    let range: std::ops::Range<usize> = 0..18446744073709551615;
    let result = with_rng(|r| r.usize(range));
}

#[test]
fn test_rng_isize() {
    let range: std::ops::Range<isize> = -9223372036854775808..9223372036854775807;
    let result = with_rng(|r| r.isize(range));
}

#[test]
fn test_rng_char() {
    let range: std::ops::RangeInclusive<char> = '\u{0}'..='\u{10FFFF}';
    let result = with_rng(|r| r.char(range));
}

#[should_panic]
fn test_rng_u8_empty_range() {
    let range: std::ops::Range<u8> = 255..255;
    let result = with_rng(|r| r.u8(range));
}

#[should_panic]
fn test_rng_i8_empty_range() {
    let range: std::ops::Range<i8> = 127..127;
    let result = with_rng(|r| r.i8(range));
}

#[should_panic]
fn test_rng_u16_empty_range() {
    let range: std::ops::Range<u16> = 65535..65535;
    let result = with_rng(|r| r.u16(range));
}

#[should_panic]
fn test_rng_i16_empty_range() {
    let range: std::ops::Range<i16> = 32767..32767;
    let result = with_rng(|r| r.i16(range));
}

#[should_panic]
fn test_rng_u32_empty_range() {
    let range: std::ops::Range<u32> = 4294967295..4294967295;
    let result = with_rng(|r| r.u32(range));
}

#[should_panic]
fn test_rng_i32_empty_range() {
    let range: std::ops::Range<i32> = 2147483647..2147483647;
    let result = with_rng(|r| r.i32(range));
}

#[should_panic]
fn test_rng_u64_empty_range() {
    let range: std::ops::Range<u64> = 18446744073709551615..18446744073709551615;
    let result = with_rng(|r| r.u64(range));
}

#[should_panic]
fn test_rng_i64_empty_range() {
    let range: std::ops::Range<i64> = 9223372036854775807..9223372036854775807;
    let result = with_rng(|r| r.i64(range));
}

#[should_panic]
fn test_rng_u128_empty_range() {
    let range: std::ops::Range<u128> = 340282366920938463463374607431768211455..340282366920938463463374607431768211455;
    let result = with_rng(|r| r.u128(range));
}

#[should_panic]
fn test_rng_i128_empty_range() {
    let range: std::ops::Range<i128> = 170141183460469231731687303715884105727..170141183460469231731687303715884105727;
    let result = with_rng(|r| r.i128(range));
}

#[should_panic]
fn test_rng_usize_empty_range() {
    let range: std::ops::Range<usize> = 18446744073709551615..18446744073709551615;
    let result = with_rng(|r| r.usize(range));
}

#[should_panic]
fn test_rng_isize_empty_range() {
    let range: std::ops::Range<isize> = 9223372036854775807..9223372036854775807;
    let result = with_rng(|r| r.isize(range));
}

#[should_panic]
fn test_rng_char_empty_range() {
    let range: std::ops::RangeInclusive<char> = '\u{10FFFF}'..='\u{10FFFF}';
    let result = with_rng(|r| r.char(range));
}

