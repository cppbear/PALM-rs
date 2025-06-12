// Answer 0

#[derive(Debug, PartialEq)]
enum Danger {
    Red,
    Yellow,
    Green,
}

struct TestStruct {
    danger: Danger,
}

impl TestStruct {
    fn is_yellow(&self) -> bool {
        matches!(*self, TestStruct { danger: Danger::Yellow })
    }
}

#[test]
fn test_is_yellow_returns_false_for_non_yellow() {
    let non_yellow_danger = TestStruct { danger: Danger::Red };
    assert_eq!(non_yellow_danger.is_yellow(), false);
}

#[test]
fn test_is_yellow_returns_false_for_another_non_yellow() {
    let another_non_yellow_danger = TestStruct { danger: Danger::Green };
    assert_eq!(another_non_yellow_danger.is_yellow(), false);
}

