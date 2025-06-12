// Answer 0

#[test]
fn test_expand_bytes_empty_replacement() {
    use re_bytes::{Captures, Ref};

    struct DummyCaptures {
        data: Vec<Option<&'static str>>,
    }

    impl Captures for DummyCaptures {
        fn get(&self, index: usize) -> Option<&str> {
            self.data.get(index).and_then(|&opt| opt)
        }

        fn name(&self, _name: &str) -> Option<&str> {
            None
        }
    }

    let caps = DummyCaptures {
        data: vec![Some("test")],
    };
    let replacement: &[u8] = &[];
    let mut dst = Vec::new();

    expand_bytes(&caps, replacement, &mut dst);

    assert_eq!(dst, b"");
}

#[test]
#[should_panic]
fn test_expand_bytes_invalid_capture_index() {
    use re_bytes::{Captures, Ref};

    struct DummyCaptures {
        data: Vec<Option<&'static str>>,
    }

    impl Captures for DummyCaptures {
        fn get(&self, index: usize) -> Option<&str> {
            self.data.get(index).and_then(|&opt| opt)
        }

        fn name(&self, _name: &str) -> Option<&str> {
            None
        }
    }

    let caps = DummyCaptures {
        data: vec![None],
    };
    let replacement: &[u8] = b"$1"; // Invalid index reference
    let mut dst = Vec::new();

    expand_bytes(&caps, replacement, &mut dst);
}

