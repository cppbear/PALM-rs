// Answer 0

#[test]
fn test_fmt_max_size_reached() {
    struct MaxSizeReached {
        _priv: (),
    }

    impl fmt::Debug for MaxSizeReached {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("MaxSizeReached")
                .finish()
        }
    }

    let instance = MaxSizeReached { _priv: () };
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    
    let result = instance.fmt(formatter);
    assert!(result.is_ok());
    assert_eq!(output, "MaxSizeReached");
}

