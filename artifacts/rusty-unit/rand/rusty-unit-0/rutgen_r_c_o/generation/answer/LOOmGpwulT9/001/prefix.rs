// Answer 0

#[test]
fn test_fill_via_u64_chunks_full_fill() {
    let mut src = [0u64, 1, 2, 3];
    let mut dest = [0u8; 15];
    fill_via_u64_chunks(&mut src, &mut dest);
}

#[test]
fn test_fill_via_u64_chunks_partial_fill() {
    let mut src = [0u64, 1];
    let mut dest = [0u8; 10];
    fill_via_u64_chunks(&mut src, &mut dest);
}

#[test]
fn test_fill_via_u64_chunks_no_fill() {
    let mut src = [0u64];
    let mut dest = [0u8; 5];
    fill_via_u64_chunks(&mut src, &mut dest);
}

#[test]
fn test_fill_via_u64_chunks_exceeding_capacity() {
    let mut src = [0u64; 2];
    let mut dest = [0u8; 3];
    fill_via_u64_chunks(&mut src, &mut dest);
}

#[test]
fn test_fill_via_u64_chunks_empty_src() {
    let mut src: [u64; 0] = [];
    let mut dest = [0u8; 10];
    fill_via_u64_chunks(&mut src, &mut dest);
}

#[test]
fn test_fill_via_u64_chunks_empty_dest() {
    let mut src = [0u64, 1, 2, 3];
    let mut dest: [u8; 0] = [];
    fill_via_u64_chunks(&mut src, &mut dest);
}

