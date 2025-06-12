// Answer 0

#[test]
fn test_as_u128_neg_int() {
    let num = Number { n: N::NegInt(-1) };
    let result = num.as_u128();

    let num = Number { n: N::NegInt(-2) };
    let result = num.as_u128();

    let num = Number { n: N::NegInt(-10) };
    let result = num.as_u128();

    let num = Number { n: N::NegInt(-123456789) };
    let result = num.as_u128();

    let num = Number { n: N::NegInt(i64::MIN) };
    let result = num.as_u128();
}

