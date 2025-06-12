// Answer 0

#[derive(Default)]
struct TestStruct<'a> {
    slice: &'a [i32],
    index: usize,
}

impl<'a> TestStruct<'a> {
    fn set_failed(&mut self, _failed: &mut bool) {
        self.slice = &self.slice[..self.index];
    }
}

#[test]
fn test_set_failed_valid_index() {
    let mut failed = false;
    let mut test_struct = TestStruct {
        slice: &[1, 2, 3, 4, 5],
        index: 3,
    };
    test_struct.set_failed(&mut failed);
    assert_eq!(test_struct.slice, &[1, 2, 3]);
}

#[test]
#[should_panic]
fn test_set_failed_index_out_of_bounds() {
    let mut failed = false;
    let mut test_struct = TestStruct {
        slice: &[1, 2, 3, 4, 5],
        index: 6,
    };
    test_struct.set_failed(&mut failed);
}

#[test]
fn test_set_failed_zero_index() {
    let mut failed = false;
    let mut test_struct = TestStruct {
        slice: &[1, 2, 3, 4, 5],
        index: 0,
    };
    test_struct.set_failed(&mut failed);
    assert_eq!(test_struct.slice, &[]);
}

#[test]
fn test_set_failed_empty_slice() {
    let mut failed = false;
    let mut test_struct = TestStruct {
        slice: &[],
        index: 0,
    };
    test_struct.set_failed(&mut failed);
    assert_eq!(test_struct.slice, &[]);
}

