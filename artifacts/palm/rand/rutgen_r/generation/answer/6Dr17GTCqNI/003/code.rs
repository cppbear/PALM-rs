// Answer 0

#[test]
fn test_fill_via_chunks_case_with_full_chunks() {
    struct TestType(u32);

    impl Observable for TestType {
        fn to_le_bytes(self) -> [u8; 4] {
            self.0.to_le_bytes()
        }
    }

    let src = [TestType(1), TestType(2), TestType(3)];
    let mut dest = [0u8; 12]; // 3 full chunks of 4 bytes

    let (num_chunks, byte_len) = fill_via_chunks(&src, &mut dest);
    
    assert_eq!(num_chunks, 3);
    assert_eq!(byte_len, 12);
    assert_eq!(dest, [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
}

#[test]
fn test_fill_via_chunks_case_with_partial_chunk() {
    struct TestType(u32);

    impl Observable for TestType {
        fn to_le_bytes(self) -> [u8; 4] {
            self.0.to_le_bytes()
        }
    }

    let src = [TestType(1), TestType(2)];
    let mut dest = [0u8; 8]; // 2 full chunks of 4 bytes

    let (num_chunks, byte_len) = fill_via_chunks(&src, &mut dest);
    
    assert_eq!(num_chunks, 2);
    assert_eq!(byte_len, 8);
    assert_eq!(dest, [1, 0, 0, 0, 2, 0, 0, 0]);
}

#[test]
fn test_fill_via_chunks_case_with_empty_dest() {
    struct TestType(u32);

    impl Observable for TestType {
        fn to_le_bytes(self) -> [u8; 4] {
            self.0.to_le_bytes()
        }
    }

    let src = [TestType(1), TestType(2), TestType(3)];
    let mut dest: [u8; 0] = []; // empty dest

    let (num_chunks, byte_len) = fill_via_chunks(&src, &mut dest);
    
    assert_eq!(num_chunks, 0);
    assert_eq!(byte_len, 0);
}

#[test]
fn test_fill_via_chunks_case_with_partial_fill() {
    struct TestType(u8);

    impl Observable for TestType {
        fn to_le_bytes(self) -> [u8; 1] {
            self.0.to_le_bytes()
        }
    }

    let src = [TestType(1), TestType(2)];
    let mut dest = [0u8; 2]; // only able to fill part

    let (num_chunks, byte_len) = fill_via_chunks(&src, &mut dest);
    
    assert_eq!(num_chunks, 1);
    assert_eq!(byte_len, 1);
    assert_eq!(dest, [1, 0]);
}

