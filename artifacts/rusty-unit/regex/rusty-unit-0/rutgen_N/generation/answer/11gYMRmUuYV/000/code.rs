// Answer 0

#[derive(Default)]
struct TestStruct {
    unicode: Option<bool>,
}

impl TestStruct {
    fn unicode(&self) -> bool {
        self.unicode.unwrap_or(true)
    }
}

#[test]
fn test_unicode_default_true() {
    let test_instance = TestStruct::default();
    assert_eq!(test_instance.unicode(), true);
}

#[test]
fn test_unicode_some_false() {
    let test_instance = TestStruct { unicode: Some(false) };
    assert_eq!(test_instance.unicode(), false);
}

#[test]
fn test_unicode_some_true() {
    let test_instance = TestStruct { unicode: Some(true) };
    assert_eq!(test_instance.unicode(), true);
}

