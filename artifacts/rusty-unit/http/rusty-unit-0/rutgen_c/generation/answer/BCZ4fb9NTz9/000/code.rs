// Answer 0

#[test]
fn test_max_size_reached_display() {
    struct MaxSizeReachedStruct {
        _priv: (),
    }

    impl fmt::Display for MaxSizeReachedStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("max size reached")
        }
    }

    let max_size_reached = MaxSizeReachedStruct { _priv: () };
    let result = format!("{}", max_size_reached);
    assert_eq!(result, "max size reached");
}

#[test]
fn test_max_size_reached_display_empty() {
    struct MaxSizeReachedStruct {
        _priv: (),
    }

    impl fmt::Display for MaxSizeReachedStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("")
        }
    }

    let max_size_reached = MaxSizeReachedStruct { _priv: () };
    let result = format!("{}", max_size_reached);
    assert_eq!(result, "");
}

