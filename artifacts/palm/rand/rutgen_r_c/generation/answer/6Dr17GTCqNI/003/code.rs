// Answer 0

#[derive(Copy, Clone)]
struct TestType(u32);

impl Observable for TestType {
    type Bytes = [u8; 4];
    
    fn to_le_bytes(self) -> Self::Bytes {
        self.0.to_le_bytes()
    }
}

#[test]
fn test_fill_via_chunks_full_chunks() {
    let src = [TestType(1), TestType(2), TestType(3)];
    let mut dest = [0u8; 12];
    let (num_chunks, byte_len) = fill_via_chunks(&src, &mut dest);
    
    assert_eq!(num_chunks, 3);
    assert_eq!(byte_len, 12);
    assert_eq!(&dest, &[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
}

#[test]
fn test_fill_via_chunks_partial_chunk() {
    let src = [TestType(1), TestType(2)];
    let mut dest = [0u8; 10];
    let (num_chunks, byte_len) = fill_via_chunks(&src, &mut dest);
    
    assert_eq!(num_chunks, 2);
    assert_eq!(byte_len, 8);
    assert_eq!(&dest, &[1, 0, 0, 0, 2, 0, 0, 0, 0, 0]);
}

#[test]
fn test_fill_via_chunks_no_full_chunks() {
    let src = [TestType(1)];
    let mut dest = [0u8; 4];
    let (num_chunks, byte_len) = fill_via_chunks(&src, &mut dest);
    
    assert_eq!(num_chunks, 1);
    assert_eq!(byte_len, 4);
    assert_eq!(&dest, &[1, 0, 0, 0]);
}

#[test]
fn test_fill_via_chunks_not_enough_dest_space() {
    let src = [TestType(1), TestType(2), TestType(3)];
    let mut dest = [0u8; 8];
    let (num_chunks, byte_len) = fill_via_chunks(&src, &mut dest);
    
    assert_eq!(num_chunks, 2);
    assert_eq!(byte_len, 8);
    assert_eq!(&dest, &[1, 0, 0, 0, 2, 0, 0, 0]);
}

