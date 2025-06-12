// Answer 0

#[test]
fn test_unexpected_neg_int_negative_10() {
    let number = Number { n: N::NegInt(-10) };
    number.unexpected();
}

#[test]
fn test_unexpected_neg_int_negative_5() {
    let number = Number { n: N::NegInt(-5) };
    number.unexpected();
}

#[test]
fn test_unexpected_neg_int_negative_1() {
    let number = Number { n: N::NegInt(-1) };
    number.unexpected();
}

