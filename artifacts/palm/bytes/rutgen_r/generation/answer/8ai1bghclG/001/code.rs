// Answer 0

#[test]
fn test_into_inner_with_basic_values() {
    struct Chain<'a, T: 'a, U: 'a> {
        a: &'a [T],
        b: &'a [U],
    }

    impl<'a, T: 'a, U: 'a> Chain<'a, T, U> {
        pub fn into_inner(self) -> (&'a [T], &'a [U]) {
            (self.a, self.b)
        }
    }

    let chain = Chain {
        a: &b"hello"[..],
        b: &b"world"[..],
    };

    let (first, last) = chain.into_inner();
    assert_eq!(first[..], b"hello"[..]);
    assert_eq!(last[..], b"world"[..]);
}

#[test]
fn test_into_inner_with_empty_first() {
    struct Chain<'a, T: 'a, U: 'a> {
        a: &'a [T],
        b: &'a [U],
    }

    impl<'a, T: 'a, U: 'a> Chain<'a, T, U> {
        pub fn into_inner(self) -> (&'a [T], &'a [U]) {
            (self.a, self.b)
        }
    }

    let chain = Chain {
        a: &b""[..],
        b: &b"world"[..],
    };

    let (first, last) = chain.into_inner();
    assert_eq!(first[..], b""[..]);
    assert_eq!(last[..], b"world"[..]);
}

#[test]
fn test_into_inner_with_empty_both() {
    struct Chain<'a, T: 'a, U: 'a> {
        a: &'a [T],
        b: &'a [U],
    }

    impl<'a, T: 'a, U: 'a> Chain<'a, T, U> {
        pub fn into_inner(self) -> (&'a [T], &'a [U]) {
            (self.a, self.b)
        }
    }

    let chain = Chain {
        a: &b""[..],
        b: &b""[..],
    };

    let (first, last) = chain.into_inner();
    assert_eq!(first[..], b""[..]);
    assert_eq!(last[..], b""[..]);
}

#[test]
fn test_into_inner_with_large_data() {
    struct Chain<'a, T: 'a, U: 'a> {
        a: &'a [T],
        b: &'a [U],
    }

    impl<'a, T: 'a, U: 'a> Chain<'a, T, U> {
        pub fn into_inner(self) -> (&'a [T], &'a [U]) {
            (self.a, self.b)
        }
    }

    let first_data = vec![0u8; 1_000_000]; // Large first part
    let last_data = vec![1u8; 1_000_000]; // Large second part

    let chain = Chain {
        a: &first_data[..],
        b: &last_data[..],
    };

    let (first, last) = chain.into_inner();
    assert_eq!(first.len(), 1_000_000);
    assert_eq!(last.len(), 1_000_000);
}

