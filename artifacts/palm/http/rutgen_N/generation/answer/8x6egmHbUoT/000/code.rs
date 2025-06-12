// Answer 0

#[test]
fn test_and_then_success() {
    struct Parts;
    struct Inner;

    impl Inner {
        fn and_then<F>(self, func: F) -> Result<Parts>
        where
            F: FnOnce(Parts) -> Result<Parts>,
        {
            func(Parts)
        }
    }

    struct Builder {
        inner: Inner,
    }

    impl Builder {
        fn and_then<F>(self, func: F) -> Self
        where
            F: FnOnce(Parts) -> Result<Parts>,
        {
            Builder {
                inner: self.inner.and_then(func).unwrap(),
            }
        }
    }

    let builder = Builder { inner: Inner };
    let new_builder = builder.and_then(|parts| Ok(parts));
    assert!(new_builder.inner.and_then(|parts| Ok(parts)).is_ok());
}

#[test]
#[should_panic]
fn test_and_then_failure() {
    struct Parts;
    struct Inner;

    impl Inner {
        fn and_then<F>(self, func: F) -> Result<Parts>
        where
            F: FnOnce(Parts) -> Result<Parts>,
        {
            func(Parts)
        }
    }

    struct Builder {
        inner: Inner,
    }

    impl Builder {
        fn and_then<F>(self, func: F) -> Self
        where
            F: FnOnce(Parts) -> Result<Parts>,
        {
            Builder {
                inner: self.inner.and_then(func).unwrap(),
            }
        }
    }

    let builder = Builder { inner: Inner };
    let _ = builder.and_then(|_parts| Err(()));
}

