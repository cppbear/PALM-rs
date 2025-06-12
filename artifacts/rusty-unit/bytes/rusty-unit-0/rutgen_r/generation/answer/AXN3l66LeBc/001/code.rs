// Answer 0

#[test]
fn test_len_empty_slice() {
    struct TestBufMut<'a>(&'a [u8]);

    let data: &mut [u8] = &mut [];
    let buf_mut = TestBufMut(data);
    assert_eq!(buf_mut.len(), 0);
}

#[test]
fn test_len_non_empty_slice() {
    struct TestBufMut<'a>(&'a [u8]);

    let data: &mut [u8] = &mut [0, 1, 2];
    let buf_mut = TestBufMut(data);
    assert_eq!(buf_mut.len(), 3);
}

#[test]
fn test_len_single_element_slice() {
    struct TestBufMut<'a>(&'a [u8]);

    let data: &mut [u8] = &mut [42];
    let buf_mut = TestBufMut(data);
    assert_eq!(buf_mut.len(), 1);
} 

#[test]
fn test_len_large_slice() {
    struct TestBufMut<'a>(&'a [u8]);

    let data: &mut [u8] = &mut [0; 1000]; // slice with 1000 zeroes
    let buf_mut = TestBufMut(data);
    assert_eq!(buf_mut.len(), 1000);
}

