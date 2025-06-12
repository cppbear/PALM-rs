// Answer 0

#[test]
fn test_value_is_null_with_null() {
    struct Value {
        data: Option<()>,
    }

    impl Value {
        fn as_null(&self) -> Option<()> {
            self.data.clone()
        }

        fn is_null(&self) -> bool {
            self.as_null().is_some()
        }
    }

    let v = Value { data: None };
    assert!(v.is_null());
}

#[test]
fn test_value_is_null_with_non_null() {
    struct Value {
        data: Option<()>,
    }

    impl Value {
        fn as_null(&self) -> Option<()> {
            self.data.clone()
        }

        fn is_null(&self) -> bool {
            self.as_null().is_some()
        }
    }

    let v = Value { data: Some(()) };
    assert!(!v.is_null());
}

