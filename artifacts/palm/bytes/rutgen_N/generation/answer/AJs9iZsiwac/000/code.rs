// Answer 0

#[test]
fn test_as_uninit_slice_mut_success() {
    struct TestBufMut {
        buffer: [u8; 3],
    }

    impl TestBufMut {
        pub fn new() -> Self {
            TestBufMut { buffer: [0, 1, 2] }
        }

        pub fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.buffer
        }
    }

    let mut buf_mut = TestBufMut::new();
    let slice = buf_mut.chunk_mut();

    unsafe {
        let uninit_slice: &mut [std::mem::MaybeUninit<u8>] = std::slice::from_mut(slice);
        uninit_slice[0].write(10);
        uninit_slice[1].write(20);
        uninit_slice[2].write(30);
        assert_eq!(buf_mut.buffer, [10, 20, 30]);
    }
}

#[test]
#[should_panic]
fn test_as_uninit_slice_mut_invalid_write() {
    struct TestBufMut {
        buffer: [u8; 3],
    }

    impl TestBufMut {
        pub fn new() -> Self {
            TestBufMut { buffer: [0, 1, 2] }
        }

        pub fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.buffer
        }
    }

    let mut buf_mut = TestBufMut::new();
    let slice = buf_mut.chunk_mut();

    unsafe {
        let uninit_slice: &mut [std::mem::MaybeUninit<u8>] = std::slice::from_mut(slice);
        // Writing uninitialized value to the slice, simulating invalid usage
        uninit_slice[0] = std::mem::MaybeUninit::uninit(); // This should cause panic or UB
    }
}

