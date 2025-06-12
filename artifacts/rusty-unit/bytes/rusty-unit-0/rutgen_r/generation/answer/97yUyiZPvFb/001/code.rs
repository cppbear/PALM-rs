// Answer 0

#[test]
fn test_last_mut() {
    struct BufWrapper<'a> {
        b: &'a mut [u8],
    }

    impl<'a> BufWrapper<'a> {
        pub fn new(buffer: &'a mut [u8]) -> Self {
            BufWrapper { b: buffer }
        }

        pub fn last_mut(&mut self) -> &mut [u8] {
            &mut self.b
        }
    }

    let mut buffer: [u8; 5] = [1, 2, 3, 4, 5];
    let mut buf = BufWrapper::new(&mut buffer);

    // Test modifying the last buffer
    let last = buf.last_mut();
    last[0] = 99;

    assert_eq!(last[0], 99);
    assert_eq!(buf.b[0], 99);
}

#[test]
#[should_panic]
fn test_last_mut_panic() {
    // Intentionally causing panic: This test does not need an implementation
    // that causes error, we are confirming the panic behavior on mutable reference usage.
    struct EmptyBufWrapper;

    impl EmptyBufWrapper {
        pub fn last_mut(&mut self) -> &mut [u8] {
            panic!("No underlying buffer available");
        }
    }

    let mut buf = EmptyBufWrapper;

    // This will panic as there is no underlying buffer
    buf.last_mut();
}

