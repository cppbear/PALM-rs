// Answer 0

#[derive(Debug)]
struct StateFlags(u8);

struct TestStruct {
    data: Vec<u8>,
}

impl TestStruct {
    fn flags(&self) -> StateFlags {
        StateFlags(self.data[0])
    }
}

#[test]
fn test_flags_returns_correct_state_flag() {
    let test_instance = TestStruct { data: vec![5] };
    let result = test_instance.flags();
    assert_eq!(result.0, 5);
}

#[test]
fn test_flags_returns_zero_for_empty_data() {
    let test_instance = TestStruct { data: vec![] };
    let result = test_instance.flags();
    assert_eq!(result.0, 0);
}

