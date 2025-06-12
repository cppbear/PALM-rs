// Answer 0

#[test]
fn test_put_slice_insufficient_capacity() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 5];
    let input = b"hello world";
    unsafe {
        buf.put_slice(input);
    }
}

#[test]
fn test_put_slice_exact_capacity() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 5];
    let input = b"hello";
    unsafe {
        buf.put_slice(input);
    }
}

#[test]
fn test_put_slice_one_byte_short() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 6];
    let input = b"hello!";
    unsafe {
        buf.put_slice(&input[..5]); // Only putting "hello"
    }
}

#[test]
fn test_put_slice_empty_input() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 5];
    let input: &[u8] = &[];
    unsafe {
        buf.put_slice(input);
    }
}

#[test]
fn test_put_slice_minimum_input() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 2];
    let input = b"a";
    unsafe {
        buf.put_slice(input);
    }
}

#[test]
fn test_put_slice_with_panic() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 0];
    let input = b"data";
    let result = std::panic::catch_unwind(|| {
        unsafe {
            buf.put_slice(input);
        }
    });
    assert!(result.is_err());
}

