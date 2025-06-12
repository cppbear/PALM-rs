// Answer 0

#[derive(Default)]
struct Error {
    column: usize,
}

struct TestStruct {
    err: Error,
}

impl TestStruct {
    pub fn column(&self) -> usize {
        self.err.column
    }
}

#[test]
fn test_column_zero() {
    let test_struct = TestStruct { err: Error { column: 0 } };
    assert_eq!(test_struct.column(), 0);
}

#[test]
fn test_column_one() {
    let test_struct = TestStruct { err: Error { column: 1 } };
    assert_eq!(test_struct.column(), 1);
}

#[test]
fn test_column_large_value() {
    let test_struct = TestStruct { err: Error { column: 100_000 } };
    assert_eq!(test_struct.column(), 100_000);
}

#[test]
fn test_column_mixed_values() {
    let test_struct_zero = TestStruct { err: Error { column: 0 } };
    let test_struct_one = TestStruct { err: Error { column: 1 } };
    let test_struct_large = TestStruct { err: Error { column: 500 } };

    assert_eq!(test_struct_zero.column(), 0);
    assert_eq!(test_struct_one.column(), 1);
    assert_eq!(test_struct_large.column(), 500);
}

