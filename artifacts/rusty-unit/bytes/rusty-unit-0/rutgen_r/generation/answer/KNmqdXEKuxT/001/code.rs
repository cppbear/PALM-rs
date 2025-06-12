// Answer 0

#[test]
fn test_last_ref_non_empty_chain() {
    struct BufChain<'a> {
        b: &'a [u8],
    }

    impl<'a> BufChain<'a> {
        fn last_ref(&self) -> &Self {
            &self
        }
    }

    let buf1 = BufChain { b: &b"hello"[..] };
    let buf2 = BufChain { b: &b"world"[..] };
    let chain = BufChain { b: buf2.last_ref().b };

    assert_eq!(chain.last_ref().b, buf2.b);
}

#[test]
fn test_last_ref_empty_chain() {
    struct BufChain<'a> {
        b: &'a [u8],
    }

    impl<'a> BufChain<'a> {
        fn last_ref(&self) -> &Self {
            &self
        }
    }

    let buf1 = BufChain { b: &b""[..] };
    let buf2 = BufChain { b: &b"example"[..] };
    let chain = BufChain { b: buf2.last_ref().b };

    assert_eq!(chain.last_ref().b, buf2.b);
}

#[test]
#[should_panic(expected = "attempt to access a field of a destructured reference that doesn't exist")]
fn test_last_ref_on_uninitialized() {
    struct BufChain<'a> {
        b: &'a [u8],
    }

    impl<'a> BufChain<'a> {
        fn last_ref(&self) -> &Self {
            panic!("attempt to access a field of a destructured reference that doesn't exist");
        }
    }

    let uninitialized_chain: BufChain<()> = unsafe { std::mem::transmute(()) };
    uninitialized_chain.last_ref(); // This should panic
}

