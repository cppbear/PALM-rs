// Answer 0

fn set_failed(slice: &[u8], index: usize) -> Vec<u8> {
    let mut failed = false;
    let mut my_struct = MyStruct {
        slice,
        index,
    };
    
    my_struct.set_failed(&mut failed);
    my_struct.slice.to_vec()
}

struct MyStruct<'a> {
    slice: &'a [u8],
    index: usize,
}

impl<'a> MyStruct<'a> {
    fn set_failed(&mut self, _failed: &mut bool) {
        self.slice = &self.slice[..self.index];
    }
}

#[test]
fn test_set_failed_valid_index() {
    let slice = b"Hello, World!";
    let index = 5;
    let result = set_failed(slice, index);
    assert_eq!(result, b"Hello".to_vec());
}

#[test]
#[should_panic]
fn test_set_failed_out_of_bounds_index() {
    let slice = b"Hello, World!";
    let index = 15; // Out of bounds
    let _result = set_failed(slice, index);
}

#[test]
fn test_set_failed_zero_index() {
    let slice = b"Hello, World!";
    let index = 0;
    let result = set_failed(slice, index);
    assert_eq!(result, b""i.to_vec());
}

#[test]
fn test_set_failed_empty_slice() {
    let slice: &[u8] = b"";
    let index = 0;
    let result = set_failed(slice, index);
    assert_eq!(result, b"".to_vec());
}

