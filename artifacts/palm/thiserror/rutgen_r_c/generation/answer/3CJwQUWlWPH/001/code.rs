// Answer 0

#[test]
fn test_fmt_with_valid_pointer() {
    struct MyPointer;
    impl Pointer for MyPointer {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "MyPointer")
        }
    }

    let value = MyPointer;
    let var = Var(&value);
    let mut output = String::new();
    let result = write!(&mut output, "Pointer: {:?}", var);

    assert!(result.is_ok());
    assert_eq!(output, "Pointer: MyPointer");
}

#[test]
fn test_fmt_with_null_pointer() {
    struct NullPointer;
    impl Pointer for NullPointer {
        fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
            panic!("This is a panic condition")
        }
    }

    let value = NullPointer;
    let var = Var(&value);

    let result = std::panic::catch_unwind(|| {
        let _ = write!(std::fmt::Formatter::new(), "Pointer: {:?}", var);
    });

    assert!(result.is_err());
}

