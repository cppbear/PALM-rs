// Answer 0

#[test]
fn test_fmt_output_slice_too_small() {
    struct ErrorCase;

    impl std::fmt::Display for ErrorCase {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Some error message")
        }
    }

    enum MyEnum {
        DecodeError(ErrorCase),
        OutputSliceTooSmall,
    }

    let case = MyEnum::OutputSliceTooSmall;
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", case));
    
    assert!(result.is_ok());
    assert_eq!(output, "Output slice too small");
}

