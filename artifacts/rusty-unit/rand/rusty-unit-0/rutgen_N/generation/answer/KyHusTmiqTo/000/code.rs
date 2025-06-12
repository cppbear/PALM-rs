// Answer 0

#[derive(Default)]
struct TestStruct(u32);

impl TestStruct {
    fn try_next_u32(&mut self) -> Result<u32, ()> {
        let value = self.0;
        self.0 += 1; // Increment for the next call
        Ok(value)
    }
}

impl TestStruct {
    fn next_u32(&mut self) -> u32 {
        self.try_next_u32().unwrap()
    }
}

#[test]
fn test_next_u32() {
    let mut test_struct = TestStruct::default();
    
    assert_eq!(test_struct.next_u32(), 0);
    assert_eq!(test_struct.next_u32(), 1);
    assert_eq!(test_struct.next_u32(), 2);
}

#[test]
fn test_next_u32_boundaries() {
    let mut test_struct = TestStruct(0);
    
    for expected in 0u32..10 {
        assert_eq!(test_struct.next_u32(), expected);
    }
}

