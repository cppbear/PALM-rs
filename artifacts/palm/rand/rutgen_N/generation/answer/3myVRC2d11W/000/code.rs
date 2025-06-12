// Answer 0

#[test]
fn test_len_empty_slice() {
    struct SliceWrapper<'a> {
        data: &'a [u8],
    }

    impl<'a> SliceWrapper<'a> {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let empty_slice = SliceWrapper { data: &[] };
    assert_eq!(empty_slice.len(), 0);
}

#[test]
fn test_len_non_empty_slice() {
    struct SliceWrapper<'a> {
        data: &'a [u8],
    }

    impl<'a> SliceWrapper<'a> {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let non_empty_slice = SliceWrapper { data: &[1, 2, 3] };
    assert_eq!(non_empty_slice.len(), 3);
}

#[test]
fn test_len_large_slice() {
    struct SliceWrapper<'a> {
        data: &'a [u8],
    }

    impl<'a> SliceWrapper<'a> {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let large_slice = SliceWrapper { data: &[0; 1000] };
    assert_eq!(large_slice.len(), 1000);
}

