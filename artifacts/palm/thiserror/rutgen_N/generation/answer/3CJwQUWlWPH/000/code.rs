// Answer 0

#[test]
fn test_pointer_fmt() {
    use std::fmt;
    
    struct Pointer<T>(T);
    
    impl<T: fmt::Debug> Pointer<T> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            Pointer::fmt(&self.0, formatter)
        }
    }

    let value = Pointer("test");
    let mut buffer = String::new();
    let result = fmt::write(&mut buffer, |f| value.fmt(f));
    assert!(result.is_ok());
    assert_eq!(buffer, "\"test\"");
}

