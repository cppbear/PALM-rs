// Answer 0

#[derive(Debug)]
struct Bytes {
    ptr: *mut u8,
    len: usize,
}

impl Bytes {
    fn new(len: usize) -> Self {
        let ptr = unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(len, 1).unwrap()) };
        Bytes { ptr, len }
    }
    
    unsafe fn inc_start(&mut self, by: usize) {
        debug_assert!(self.len >= by, "internal: inc_start out of bounds");
        self.len -= by;
        self.ptr = self.ptr.add(by);
    }
    
    fn len(&self) -> usize {
        self.len
    }
}

#[test]
fn test_inc_start_valid() {
    let mut bytes = Bytes::new(10);
    unsafe {
        bytes.inc_start(5);
    }
    assert_eq!(bytes.len(), 5);
}

#[test]
#[should_panic(expected = "internal: inc_start out of bounds")]
fn test_inc_start_out_of_bounds() {
    let mut bytes = Bytes::new(3);
    unsafe {
        bytes.inc_start(5);
    }
}

