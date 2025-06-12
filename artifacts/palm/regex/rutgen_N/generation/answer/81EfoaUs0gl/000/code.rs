// Answer 0

#[derive(Debug)]
struct CharWrapper(u32);

impl CharWrapper {
    pub fn is_none(self) -> bool {
        self.0 == u32::MAX
    }
}

#[test]
fn test_is_none_with_max_value() {
    let char_wrapper = CharWrapper(u32::MAX);
    assert!(char_wrapper.is_none());
}

#[test]
fn test_is_none_with_non_max_value() {
    let char_wrapper = CharWrapper(100);
    assert!(!char_wrapper.is_none());
}

#[test]
fn test_is_none_with_zero() {
    let char_wrapper = CharWrapper(0);
    assert!(!char_wrapper.is_none());
}

