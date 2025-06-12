// Answer 0

#[test]
fn test_fmt() {
    struct OnceRef {
        inner: i32,
    }
    
    let once_ref = OnceRef { inner: 42 };
    let mut output = Vec::new();
    let result = core::fmt::write(&mut output, |f| once_ref.fmt(f));
    
    assert!(result.is_ok());
    let formatted = String::from_utf8(output).unwrap();
    assert_eq!(formatted, "OnceRef(42)");
}

