// Answer 0

#[test]
fn test_number_fmt() {
    struct Number(i32);

    impl std::fmt::Display for Number {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "Number({})", self.0)
        }
    }

    let num_positive = Number(42);
    let num_negative = Number(-1);
    let num_zero = Number(0);

    let mut output_positive = String::new();
    let mut output_negative = String::new();
    let mut output_zero = String::new();

    assert!(num_positive.fmt(&mut output_positive).is_ok());
    assert!(num_negative.fmt(&mut output_negative).is_ok());
    assert!(num_zero.fmt(&mut output_zero).is_ok());

    assert_eq!(output_positive, "Number(42)");
    assert_eq!(output_negative, "Number(-1)");
    assert_eq!(output_zero, "Number(0)");
}

#[test]
#[should_panic]
fn test_number_fmt_panic() {
    struct PanickingNumber;

    impl std::fmt::Display for PanickingNumber {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            panic!("This is a panic!");
        }
    }

    let panicking_num = PanickingNumber;
    let _ = panicking_num.fmt(&mut String::new());
}

