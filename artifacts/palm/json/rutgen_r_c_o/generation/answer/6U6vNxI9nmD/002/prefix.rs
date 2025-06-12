// Answer 0

#[test]
fn test_as_u64_with_negative_integer() {
    let negative_number = Number { n: N::NegInt(-1) };
    negative_number.as_u64();

    let negative_number_small = Number { n: N::NegInt(-100) };
    negative_number_small.as_u64();

    let negative_number_large = Number { n: N::NegInt(-9223372036854775808) };
    negative_number_large.as_u64();
}

