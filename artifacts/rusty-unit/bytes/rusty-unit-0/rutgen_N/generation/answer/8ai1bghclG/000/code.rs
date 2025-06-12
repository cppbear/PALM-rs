// Answer 0

#[test]
fn test_into_inner_with_string_slices() {
    struct Chain<T, U> {
        a: T,
        b: U,
    }

    impl<T, U> Chain<T, U> {
        pub fn into_inner(self) -> (T, U) {
            (self.a, self.b)
        }
    }

    let chain = Chain { a: &b"hello"[..], b: &b"world"[..] };
    let (first, last) = chain.into_inner();
    assert_eq!(first[..], b"hello"[..]);
    assert_eq!(last[..], b"world"[..]);
}

#[test]
fn test_into_inner_with_empty_first() {
    struct Chain<T, U> {
        a: T,
        b: U,
    }

    impl<T, U> Chain<T, U> {
        pub fn into_inner(self) -> (T, U) {
            (self.a, self.b)
        }
    }

    let chain = Chain { a: &b""[..], b: &b"world"[..] };
    let (first, last) = chain.into_inner();
    assert_eq!(first[..], b""[..]);
    assert_eq!(last[..], b"world"[..]);
}

#[test]
fn test_into_inner_with_empty_both() {
    struct Chain<T, U> {
        a: T,
        b: U,
    }

    impl<T, U> Chain<T, U> {
        pub fn into_inner(self) -> (T, U) {
            (self.a, self.b)
        }
    }

    let chain = Chain { a: &b""[..], b: &b""[..] };
    let (first, last) = chain.into_inner();
    assert_eq!(first[..], b""[..]);
    assert_eq!(last[..], b""[..]);
}

