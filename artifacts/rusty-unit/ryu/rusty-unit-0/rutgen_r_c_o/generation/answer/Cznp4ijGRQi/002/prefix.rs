// Answer 0

#[test]
#[should_panic]
fn test_pow5factor_32_zero() {
    let value: u32 = 0;
    pow5factor_32(value);
}

