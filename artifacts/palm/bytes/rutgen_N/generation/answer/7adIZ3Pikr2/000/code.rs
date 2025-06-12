// Answer 0

#[test]
fn test_remaining_mut() {
    struct BufMut {
        data: Vec<u8>,
    }

    impl BufMut {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn remaining_mut(&self) -> usize {
            core::isize::MAX as usize - self.len()
        }
    }

    let buf = BufMut {
        data: vec![0; 10],
    };

    assert_eq!(buf.remaining_mut(), core::isize::MAX as usize - 10);
}

#[test]
fn test_remaining_mut_zero_length() {
    struct BufMut {
        data: Vec<u8>,
    }

    impl BufMut {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn remaining_mut(&self) -> usize {
            core::isize::MAX as usize - self.len()
        }
    }

    let buf = BufMut { data: Vec::new() };

    assert_eq!(buf.remaining_mut(), core::isize::MAX as usize);
}

