// Answer 0

#[test]
fn test_fill_via_chunks_with_partial_fill() {
    #[derive(Copy, Clone)]
    struct TestData {
        value: u32,
    }

    impl Observable for TestData {
        type Bytes = [u8; 4];

        fn to_le_bytes(self) -> Self::Bytes {
            self.value.to_le_bytes()
        }
    }

    let src = [TestData { value: 1 }, TestData { value: 2 }, TestData { value: 3 }];
    let mut dest = [0u8; 10]; // Size enough to fill up to 2 complete chunks of 4 bytes (8 bytes total) plus some extra space.

    let (n, byte_len) = fill_via_chunks(&src, &mut dest);

    assert_eq!(n, 3); // 2 complete chunks + 1 partial
    assert_eq!(byte_len, 10); // 8 bytes filled from chunks + 2 bytes from the last element
    assert_eq!(&dest[..8], &[1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8]); // Check first 8 bytes after first 2 full chunks
    assert_eq!(&dest[8..10], &[3u8, 0u8]); // Ensure last 2 bytes filled with part of the third element
}

#[test]
#[should_panic]
fn test_fill_via_chunks_with_dest_small() {
    #[derive(Copy, Clone)]
    struct TestData {
        value: u32,
    }

    impl Observable for TestData {
        type Bytes = [u8; 4];

        fn to_le_bytes(self) -> Self::Bytes {
            self.value.to_le_bytes()
        }
    }

    let src = [TestData { value: 1 }, TestData { value: 2 }];
    let mut dest = [0u8; 3]; // Dest is too small to fit even one complete chunk of 4 bytes

    // This should panic as dest is not large enough
    let _ = fill_via_chunks(&src, &mut dest);
}

#[test]
fn test_fill_via_chunks_with_exact_fit() {
    #[derive(Copy, Clone)]
    struct TestData {
        value: u32,
    }

    impl Observable for TestData {
        type Bytes = [u8; 4];

        fn to_le_bytes(self) -> Self::Bytes {
            self.value.to_le_bytes()
        }
    }

    let src = [TestData { value: 5 }, TestData { value: 6 }];
    let mut dest = [0u8; 8]; // Size equal to 2 complete chunks of 4 bytes each

    let (n, byte_len) = fill_via_chunks(&src, &mut dest);

    assert_eq!(n, 2); // 2 complete chunks filled
    assert_eq!(byte_len, 8); // Fully filled 8 bytes
    assert_eq!(&dest[..], &[5u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8]); // Ensure both elements are correctly filled
}

