// Answer 0

#[test]
fn test_copy_from_slice_equal_length() {
    struct UninitSlice {
        data: *mut u8,
        length: usize,
    }

    impl UninitSlice {
        unsafe fn from_raw_parts_mut(data: *mut u8, length: usize) -> Self {
            UninitSlice { data, length }
        }

        fn len(&self) -> usize {
            self.length
        }

        unsafe fn as_mut_ptr(&self) -> *mut u8 {
            self.data
        }
    }

    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    slice.copy_from_slice(b"foo");
    assert_eq!(b"foo", &data[..]);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_copy_from_slice_different_length() {
    struct UninitSlice {
        data: *mut u8,
        length: usize,
    }

    impl UninitSlice {
        unsafe fn from_raw_parts_mut(data: *mut u8, length: usize) -> Self {
            UninitSlice { data, length }
        }

        fn len(&self) -> usize {
            self.length
        }

        unsafe fn as_mut_ptr(&self) -> *mut u8 {
            self.data
        }
    }

    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    slice.copy_from_slice(b"barbaz"); // This will panic because lengths differ
}

