// Answer 0

#[derive(Debug)]
struct TestStruct<'t> {
    text: &'t [u8],
    start: usize,
    end: usize,
}

impl<'t> TestStruct<'t> {
    pub fn as_bytes(&self) -> &'t [u8] {
        &self.text[self.start..self.end]
    }
}

#[test]
fn test_as_bytes_valid_range() {
    let text: &[u8] = b"Hello, world!";
    let instance = TestStruct { text, start: 0, end: 5 };
    
    let result = instance.as_bytes();
    assert_eq!(result, b"Hello");
}

#[test]
fn test_as_bytes_full_range() {
    let text: &[u8] = b"Hello, world!";
    let instance = TestStruct { text, start: 0, end: 13 };
    
    let result = instance.as_bytes();
    assert_eq!(result, b"Hello, world!");
}

#[test]
fn test_as_bytes_empty_range() {
    let text: &[u8] = b"Hello, world!";
    let instance = TestStruct { text, start: 5, end: 5 };
    
    let result = instance.as_bytes();
    assert_eq!(result, b"");
}

#[should_panic]
#[test]
fn test_as_bytes_start_out_of_bounds() {
    let text: &[u8] = b"Hello, world!";
    let instance = TestStruct { text, start: 14, end: 15 };
    
    instance.as_bytes();
}

#[should_panic]
#[test]
fn test_as_bytes_end_out_of_bounds() {
    let text: &[u8] = b"Hello, world!";
    let instance = TestStruct { text, start: 10, end: 20 };
    
    instance.as_bytes();
}

#[should_panic]
#[test]
fn test_as_bytes_start_greater_than_end() {
    let text: &[u8] = b"Hello, world!";
    let instance = TestStruct { text, start: 5, end: 4 };
    
    instance.as_bytes();
}

