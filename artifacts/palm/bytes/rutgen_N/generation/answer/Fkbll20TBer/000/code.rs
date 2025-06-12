// Answer 0

#[derive(Debug)]
struct Bytes {
    ptr: *const u8,
    len: usize,
    data: std::sync::atomic::AtomicPtr<u8>,
    vtable: &'static str, // Assuming the vtable is a static str for this example
}

impl Bytes {
    pub const fn from_static(bytes: &'static [u8]) -> Self {
        Bytes {
            ptr: bytes.as_ptr(),
            len: bytes.len(),
            data: std::sync::atomic::AtomicPtr::new(std::ptr::null_mut()),
            vtable: "STATIC_VTABLE", // Simulating the vtable
        }
    }
}

#[test]
fn test_from_static() {
    let b = Bytes::from_static(b"hello");
    assert_eq!(unsafe { std::slice::from_raw_parts(b.ptr, b.len) }, b"hello");
}

#[test]
fn test_empty_bytes() {
    let b = Bytes::from_static(b"");
    assert_eq!(unsafe { std::slice::from_raw_parts(b.ptr, b.len) }, b"");
}

#[test]
fn test_large_bytes() {
    let input = b"this is a longer byte array for testing";
    let b = Bytes::from_static(input);
    assert_eq!(unsafe { std::slice::from_raw_parts(b.ptr, b.len) }, input);
}

