// Answer 0

#[test]
fn test_is_f64_with_float() {
    struct Number {
        n: N,
    }

    enum N {
        Float(f64),
        PosInt(u64),
        NegInt(i64),
    }

    let float_value = 3.14;
    let number = Number { n: N::Float(float_value) };

    assert_eq!(number.is_f64(), true);
}

#[test]
fn test_is_f64_with_pos_int() {
    struct Number {
        n: N,
    }

    enum N {
        Float(f64),
        PosInt(u64),
        NegInt(i64),
    }

    let int_value = 42;
    let number = Number { n: N::PosInt(int_value) };

    assert_eq!(number.is_f64(), false);
}

#[test]
fn test_is_f64_with_neg_int() {
    struct Number {
        n: N,
    }

    enum N {
        Float(f64),
        PosInt(u64),
        NegInt(i64),
    }

    let int_value = -7;
    let number = Number { n: N::NegInt(int_value) };

    assert_eq!(number.is_f64(), false);
}

#[test]
fn test_is_f64_with_arbitrary_precision() {
    #[cfg(feature = "arbitrary_precision")]
    {
        struct Number {
            n: String,
        }

        impl Number {
            pub fn is_f64(&self) -> bool {
                for c in self.n.chars() {
                    if c == '.' || c == 'e' || c == 'E' {
                        return self.n.parse::<f64>().ok().map_or(false, f64::is_finite);
                    }
                }
                false
            }
        }

        let number_finite = Number { n: String::from("3.14") };
        assert_eq!(number_finite.is_f64(), true);

        let number_infinite = Number { n: String::from("1e309") };
        assert_eq!(number_infinite.is_f64(), false);

        let number_invalid = Number { n: String::from("not a number") };
        assert_eq!(number_invalid.is_f64(), false);
    }
}

