// Answer 0

#[test]
fn test_find_end_empty_haystack() {
    struct TestLit<I> {
        iter: I,
    }

    impl<I> TestLit<I>
    where
        I: Iterator<Item = &'static [u8]>,
    {
        fn iter(&self) -> I {
            self.iter
        }
    }

    let lit = [&b"abcd"[..]];
    let test_lit = TestLit { iter: lit.iter() };
    let haystack: &[u8] = &[];

    assert_eq!(test_lit.find_end(haystack), None);
}

#[test]
fn test_find_end_haystack_small() {
    struct TestLit<I> {
        iter: I,
    }

    impl<I> TestLit<I>
    where
        I: Iterator<Item = &'static [u8]>,
    {
        fn iter(&self) -> I {
            self.iter
        }
    }

    let lit = [&b"hello"[..]];
    let test_lit = TestLit { iter: lit.iter() };
    let haystack: &[u8] = &b"hi"[..];

    assert_eq!(test_lit.find_end(haystack), None);
}

#[test]
fn test_find_end_haystack_equals_lit_length() {
    struct TestLit<I> {
        iter: I,
    }

    impl<I> TestLit<I>
    where
        I: Iterator<Item = &'static [u8]>,
    {
        fn iter(&self) -> I {
            self.iter
        }
    }

    let lit = [&b"short"[..]];
    let test_lit = TestLit { iter: lit.iter() };
    let haystack: &[u8] = &b"longer text"[..];

    assert_eq!(test_lit.find_end(haystack), None);
}

#[test]
fn test_find_end_haystack_only_lit() {
    struct TestLit<I> {
        iter: I,
    }

    impl<I> TestLit<I>
    where
        I: Iterator<Item = &'static [u8]>,
    {
        fn iter(&self) -> I {
            self.iter
        }
    }

    let lit = [&b"xyz"[..]];
    let test_lit = TestLit { iter: lit.iter() };
    let haystack: &[u8] = &b"abc"[..];

    assert_eq!(test_lit.find_end(haystack), None);
}

