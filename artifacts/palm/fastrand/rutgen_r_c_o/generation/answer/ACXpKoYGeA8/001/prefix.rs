// Answer 0

#[test]
fn test_try_with_rng_u8() {
    try_with_rng(|rng| {
        let value = rng.u8(0..=255);
    }).unwrap();
}

#[test]
fn test_try_with_rng_i8() {
    try_with_rng(|rng| {
        let value = rng.i8(-128..=127);
    }).unwrap();
}

#[test]
fn test_try_with_rng_u16() {
    try_with_rng(|rng| {
        let value = rng.u16(0..=65535);
    }).unwrap();
}

#[test]
fn test_try_with_rng_i16() {
    try_with_rng(|rng| {
        let value = rng.i16(-32768..=32767);
    }).unwrap();
}

#[test]
fn test_try_with_rng_u32() {
    try_with_rng(|rng| {
        let value = rng.u32(0..=4294967295);
    }).unwrap();
}

#[test]
fn test_try_with_rng_i32() {
    try_with_rng(|rng| {
        let value = rng.i32(-2147483648..=2147483647);
    }).unwrap();
}

#[test]
fn test_try_with_rng_u64() {
    try_with_rng(|rng| {
        let value = rng.u64(0..=18446744073709551615);
    }).unwrap();
}

#[test]
fn test_try_with_rng_i64() {
    try_with_rng(|rng| {
        let value = rng.i64(-9223372036854775808..=9223372036854775807);
    }).unwrap();
}

#[test]
fn test_try_with_rng_u128() {
    try_with_rng(|rng| {
        let value = rng.u128(0..=340282366920938463463374607431768211455);
    }).unwrap();
}

#[test]
fn test_try_with_rng_i128() {
    try_with_rng(|rng| {
        let value = rng.i128(-170141183460469231731687303715884105728..=170141183460469231731687303715884105727);
    }).unwrap();
}

#[test]
fn test_try_with_rng_usize() {
    try_with_rng(|rng| {
        let value = rng.usize(0..=usize::MAX);
    }).unwrap();
}

#[test]
fn test_try_with_rng_isize() {
    try_with_rng(|rng| {
        let value = rng.isize(isize::MIN..=isize::MAX);
    }).unwrap();
}

#[test]
fn test_try_with_rng_char() {
    try_with_rng(|rng| {
        let value = rng.char('\u{0}'..='\u{10FFFF}');
    }).unwrap();
}

