// Answer 0

#[derive(Copy, Clone)]
struct MyBytes([u8; 4]);

impl AsRef<[u8]> for MyBytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Observable for MyBytes {
    type Bytes = MyBytes;

    fn to_le_bytes(self) -> Self::Bytes {
        self
    }
}

#[test]
fn test_fill_via_chunks_full_chunks() {
    let src = [MyBytes([1, 2, 3, 4]), MyBytes([5, 6, 7, 8])];
    let mut dest = [0u8; 8];
    let (n, byte_len) = fill_via_chunks(&src, &mut dest);

    assert_eq!(n, 2);
    assert_eq!(byte_len, 8);
    assert_eq!(&dest, &[1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_fill_via_chunks_partial_chunk() {
    let src = [MyBytes([1, 2, 3, 4]), MyBytes([5, 6, 7, 8])];
    let mut dest = [0u8; 10];
    let (n, byte_len) = fill_via_chunks(&src, &mut dest);

    assert_eq!(n, 2);
    assert_eq!(byte_len, 8);
    assert_eq!(&dest[..8], &[1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_fill_via_chunks_empty_src() {
    let src: [MyBytes; 0] = [];
    let mut dest = [0u8; 8];
    let (n, byte_len) = fill_via_chunks(&src, &mut dest);

    assert_eq!(n, 0);
    assert_eq!(byte_len, 0);
    assert_eq!(&dest, &[0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_fill_via_chunks_empty_dest() {
    let src = [MyBytes([1, 2, 3, 4])];
    let mut dest = [0u8; 0];
    let (n, byte_len) = fill_via_chunks(&src, &mut dest);

    assert_eq!(n, 0);
    assert_eq!(byte_len, 0);
}

#[test]
fn test_fill_via_chunks_source_larger_than_dest() {
    let src = [MyBytes([1, 2, 3, 4]), MyBytes([5, 6, 7, 8])];
    let mut dest = [0u8; 4]; // smaller destination
    let (n, byte_len) = fill_via_chunks(&src, &mut dest);

    assert_eq!(n, 1);
    assert_eq!(byte_len, 4);
    assert_eq!(&dest, &[1, 2, 3, 4]);
}

