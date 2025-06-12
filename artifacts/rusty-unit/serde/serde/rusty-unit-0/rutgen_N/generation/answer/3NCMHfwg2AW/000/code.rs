// Answer 0

#[derive(Debug)]
struct MyStruct {
    bytes: Vec<u8>,
    offset: usize,
}

impl MyStruct {
    pub fn as_str(&self) -> &str {
        let slice = &self.bytes[..self.offset];
        unsafe { std::str::from_utf8_unchecked(slice) }
    }
}

#[test]
fn test_as_str_valid() {
    let my_struct = MyStruct {
        bytes: b"hello, world".to_vec(),
        offset: 5,
    };
    assert_eq!(my_struct.as_str(), "hello");
}

#[test]
fn test_as_str_empty() {
    let my_struct = MyStruct {
        bytes: b"".to_vec(),
        offset: 0,
    };
    assert_eq!(my_struct.as_str(), "");
}

#[should_panic]
#[test]
fn test_as_str_out_of_bounds() {
    let my_struct = MyStruct {
        bytes: b"hello".to_vec(),
        offset: 10,
    };
    let _ = my_struct.as_str(); // This should panic due to out-of-bounds access
}

