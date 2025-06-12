// Answer 0

#[derive(Debug)]
enum N {
    PosInt(u32),
    NegInt(i32),
    Float(f64),
}

struct Number {
    n: N,
}

impl std::fmt::Display for Number {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.n {
            N::PosInt(u) => formatter.write_str(itoa::Buffer::new().format(u)),
            N::NegInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
            N::Float(f) => formatter.write_str(ryu::Buffer::new().format_finite(f)),
        }
    }
}

#[test]
fn test_number_format_float() {
    let number = Number { n: N::Float(3.14) };
    let result = format!("{}", number);
    assert_eq!(result, "3.14");

    let number = Number { n: N::Float(0.0) };
    let result = format!("{}", number);
    assert_eq!(result, "0");

    let number = Number { n: N::Float(-1.5) };
    let result = format!("{}", number);
    assert_eq!(result, "-1.5");

    let number = Number { n: N::Float(f64::INFINITY) };
    let result = format!("{}", number);
    assert_eq!(result, "inf");

    let number = Number { n: N::Float(f64::NEG_INFINITY) };
    let result = format!("{}", number);
    assert_eq!(result, "-inf");

    let number = Number { n: N::Float(f64::NAN) };
    let result = format!("{}", number);
    assert_eq!(result, "nan");
}

