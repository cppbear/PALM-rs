// Answer 0

#[derive(Debug)]
struct OnceRef<T> {
    inner: T,
}

impl<T: std::fmt::Debug> OnceRef<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "OnceRef({:?})", self.inner)
    }
}

#[test]
fn test_fmt_with_simple_value() {
    let value = OnceRef { inner: 42 };
    let mut output = core::fmt::Formatter::new(&mut Vec::new());
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output.into_inner(), b"OnceRef(42)");
}

#[test]
fn test_fmt_with_string() {
    let value = OnceRef { inner: String::from("test") };
    let mut output = core::fmt::Formatter::new(&mut Vec::new());
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output.into_inner(), b"OnceRef(\"test\")");
}

#[test]
fn test_fmt_with_tuple() {
    let value = OnceRef { inner: (1, "tuple") };
    let mut output = core::fmt::Formatter::new(&mut Vec::new());
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output.into_inner(), b"OnceRef((1, \"tuple\"))");
}

#[test]
fn test_fmt_with_struct() {
    #[derive(Debug)]
    struct TestStruct {
        value: i32,
    }
    let value = OnceRef { inner: TestStruct { value: 5 } };
    let mut output = core::fmt::Formatter::new(&mut Vec::new());
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output.into_inner(), b"OnceRef(TestStruct { value: 5 })");
}

