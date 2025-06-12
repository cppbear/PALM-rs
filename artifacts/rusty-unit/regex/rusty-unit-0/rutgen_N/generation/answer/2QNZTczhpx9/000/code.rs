// Answer 0

#[derive(Debug)]
struct MyStruct(Vec<u8>);

impl MyStruct {
    fn find_at(&self, text: &[u8], start: usize) -> Option<(usize, usize)> {
        // Example implementation (to be replaced with actual logic)
        if start < text.len() {
            Some((start, start + 1))
        } else {
            None
        }
    }
}

#[test]
fn test_find_at_valid_start() {
    let my_struct = MyStruct(b"hello".to_vec());
    let result = my_struct.find_at(b"hello", 0);
    assert_eq!(result, Some((0, 1)));
}

#[test]
fn test_find_at_valid_start_middle() {
    let my_struct = MyStruct(b"hello".to_vec());
    let result = my_struct.find_at(b"hello", 2);
    assert_eq!(result, Some((2, 3)));
}

#[test]
fn test_find_at_invalid_start() {
    let my_struct = MyStruct(b"hello".to_vec());
    let result = my_struct.find_at(b"hello", 5);
    assert_eq!(result, None);
}

#[test]
fn test_find_at_start_out_of_bounds() {
    let my_struct = MyStruct(b"hello".to_vec());
    let result = my_struct.find_at(b"hello", 10);
    assert_eq!(result, None);
}

