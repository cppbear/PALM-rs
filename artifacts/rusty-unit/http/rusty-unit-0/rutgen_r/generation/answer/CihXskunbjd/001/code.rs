// Answer 0

#[test]
fn test_empty_scheme() {
    struct Scheme {
        inner: Scheme2,
    }

    enum Scheme2 {
        None,
    }

    impl Scheme {
        pub(super) fn empty() -> Self {
            Scheme {
                inner: Scheme2::None,
            }
        }
    }

    let scheme = Scheme::empty();
    match scheme.inner {
        Scheme2::None => {}
    }
}

