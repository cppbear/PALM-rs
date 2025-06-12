// Answer 0

#[test]
fn test_is_f64_with_negative_integer() {
    let number1 = Number { n: N::NegInt(-1) };
    let number2 = Number { n: N::NegInt(-2) };
    let number3 = Number { n: N::NegInt(-3) };
    let number4 = Number { n: N::NegInt(-4) };
    let number5 = Number { n: N::NegInt(-5) };
    let number6 = Number { n: N::NegInt(-6) };
    let number7 = Number { n: N::NegInt(-7) };
    let number8 = Number { n: N::NegInt(-8) };
    let number9 = Number { n: N::NegInt(-9) };
    let number10 = Number { n: N::NegInt(-10) };

    number1.is_f64();
    number2.is_f64();
    number3.is_f64();
    number4.is_f64();
    number5.is_f64();
    number6.is_f64();
    number7.is_f64();
    number8.is_f64();
    number9.is_f64();
    number10.is_f64();
}

