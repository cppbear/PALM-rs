// Answer 0

#[test]
fn test_chunks_vectored_empty_self_empty_dst() {
    let buffer: VecDeque<u8> = VecDeque::new();
    let mut dst: [io::IoSlice; 2] = Default::default();
    buffer.chunks_vectored(&mut dst);
}

#[test]
fn test_chunks_vectored_empty_self_non_empty_dst() {
    let buffer: VecDeque<u8> = VecDeque::new();
    let mut dst: [io::IoSlice; 2] = Default::default();
    dst[0] = io::IoSlice::new(&[]);
    buffer.chunks_vectored(&mut dst);
}

#[test]
fn test_chunks_vectored_non_empty_self_empty_dst() {
    let buffer: VecDeque<u8> = VecDeque::from(vec![1, 2, 3]);
    let mut dst: [io::IoSlice; 2] = Default::default();
    buffer.chunks_vectored(&mut dst);
}

