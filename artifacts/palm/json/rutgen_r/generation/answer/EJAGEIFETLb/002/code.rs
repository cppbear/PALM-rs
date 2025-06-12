// Answer 0

#[test]
fn test_byte_offset_none() {
    struct Iter {
        offset: usize,
    }

    impl Iter {
        fn byte_offset(&self) -> usize {
            self.offset
        }
    }

    struct Context {
        ch: Option<()>,
        iter: Iter,
    }

    let context = Context {
        ch: None,
        iter: Iter { offset: 5 },
    };
    
    assert_eq!(context.byte_offset(), 5);
}

#[test]
fn test_byte_offset_none_zero_offset() {
    struct Iter {
        offset: usize,
    }

    impl Iter {
        fn byte_offset(&self) -> usize {
            self.offset
        }
    }

    struct Context {
        ch: Option<()>,
        iter: Iter,
    }

    let context = Context {
        ch: None,
        iter: Iter { offset: 0 },
    };
    
    assert_eq!(context.byte_offset(), 0);
}

