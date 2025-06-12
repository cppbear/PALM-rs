// Answer 0

#[cfg(test)]
struct TestStruct;

impl TestStruct {
    pub fn len(&self) -> usize { 0 }
}

#[test]
fn test_len_returns_zero() {
    let test_struct = TestStruct;
    assert_eq!(test_struct.len(), 0);
}

