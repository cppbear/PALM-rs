// Answer 0

#[test]
fn test_fill_via_chunks_full_chunks() {
    struct TestObservable(u32);
    
    impl Observable for TestObservable {
        fn to_le_bytes(&self) -> [u8; std::mem::size_of::<u32>()] {
            self.0.to_le_bytes()
        }
    }

    let src = [TestObservable(1), TestObservable(2), TestObservable(3)];
    let mut dest = [0u8; 12]; // 3 elements * 4 bytes each
    let (n, byte_len) = fill_via_chunks(&src, &mut dest);
    
    assert_eq!(n, 3);
    assert_eq!(byte_len, 12);
    assert_eq!(dest, [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
}

#[test]
fn test_fill_via_chunks_partial_chunks() {
    struct TestObservable(u32);
    
    impl Observable for TestObservable {
        fn to_le_bytes(&self) -> [u8; std::mem::size_of::<u32>()] {
            self.0.to_le_bytes()
        }
    }

    let src = [TestObservable(1), TestObservable(2)];
    let mut dest = [0u8; 10]; // 2 elements * 4 bytes + 2 bytes for the remainder
    let (n, byte_len) = fill_via_chunks(&src, &mut dest);
    
    assert_eq!(n, 2);
    assert_eq!(byte_len, 8); // Only full chunks
    assert_eq!(dest, [1, 0, 0, 0, 2, 0, 0, 0, 0, 0]); // Last two bytes remain unchanged
}

#[test]
fn test_fill_via_chunks_empty_src() {
    struct TestObservable(u32);
    
    impl Observable for TestObservable {
        fn to_le_bytes(&self) -> [u8; std::mem::size_of::<u32>()] {
            self.0.to_le_bytes()
        }
    }

    let src: [TestObservable; 0] = [];
    let mut dest = [0u8; 8];
    let (n, byte_len) = fill_via_chunks(&src, &mut dest);
    
    assert_eq!(n, 0);
    assert_eq!(byte_len, 0);
    assert_eq!(dest, [0, 0, 0, 0, 0, 0, 0, 0]); // Dest should remain unchanged as src is empty
}

#[test]
fn test_fill_via_chunks_dest_shorter_than_src() {
    struct TestObservable(u32);
    
    impl Observable for TestObservable {
        fn to_le_bytes(&self) -> [u8; std::mem::size_of::<u32>()] {
            self.0.to_le_bytes()
        }
    }

    let src = [TestObservable(1), TestObservable(2), TestObservable(3)];
    let mut dest = [0u8; 5]; // Shorter destination
    let (n, byte_len) = fill_via_chunks(&src, &mut dest);
    
    assert_eq!(n, 2); // Only filled 2 chunks
    assert_eq!(byte_len, 8); // 2 full chunks filled (4 + 4 bytes)
    assert_eq!(dest, [1, 0, 0, 0, 2]); // Last byte from the 3rd element cannot be added
}

