// Answer 0

#[test]
#[should_panic]
fn test_pow5bits_negative_input() {
    let e = -1;
    pow5bits(e);
}

