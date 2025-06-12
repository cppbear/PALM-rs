// Answer 0

#[derive(Debug)]
struct Bytes {
    len: usize,
    ptr: *mut u8,
}

impl Bytes {
    fn new(len: usize) -> Self {
        let ptr = unsafe { std::alloc::alloc(std::alloc::Layout::array::<u8>(len).unwrap()) };
        Bytes { len, ptr }
    }
}

#[test]
#[should_panic(expected = "internal: inc_start out of bounds")]
fn test_inc_start_out_of_bounds() {
    let mut bytes = Bytes::new(5); // len is 5
    unsafe {
        bytes.inc_start(10); // by is 10 which exceeds len
    }
}

#[test]
fn test_inc_start_exact_bounds() {
    let mut bytes = Bytes::new(5); // len is 5
    unsafe {
        bytes.inc_start(5); // by is 5 which equals len
    }
    assert_eq!(bytes.len, 0); // Expect len to be reduced to 0
}

#[test]
fn test_inc_start_zero() {
    let mut bytes = Bytes::new(5); // len is 5
    unsafe {
        bytes.inc_start(0); // by is 0 which should not change anything
    }
    assert_eq!(bytes.len, 5); // Expect len to remain 5
}

