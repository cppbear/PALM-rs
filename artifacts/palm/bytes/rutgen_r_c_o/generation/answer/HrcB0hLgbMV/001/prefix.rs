// Answer 0

#[test]
fn test_chunk_mut_with_zero_capacity() {
    let mut vec: Vec<u8> = Vec::with_capacity(0);
    let _result = vec.chunk_mut();
}

#[test]
fn test_chunk_mut_with_exact_capacity() {
    let mut vec: Vec<u8> = Vec::with_capacity(64);
    vec.resize(64, 0);
    let _result = vec.chunk_mut();
}

#[test]
fn test_chunk_mut_with_small_capacity() {
    let mut vec: Vec<u8> = Vec::with_capacity(32);
    vec.resize(32, 0);
    let _result = vec.chunk_mut();
}

#[test]
fn test_chunk_mut_with_half_capacity() {
    let mut vec: Vec<u8> = Vec::with_capacity(128);
    vec.resize(64, 0);
    let _result = vec.chunk_mut();
}

#[test]
fn test_chunk_mut_with_capacity_greater_than_size() {
    let mut vec: Vec<u8> = Vec::with_capacity(128);
    vec.resize(64, 0);
    let _result = vec.chunk_mut();
}

