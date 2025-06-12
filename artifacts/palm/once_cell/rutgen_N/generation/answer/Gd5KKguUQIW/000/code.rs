// Answer 0

#[cfg(test)]
struct Lazy<T> {
    value: Option<T>,
}

impl<T> Lazy<T> {
    fn force_mut(&mut self) -> &mut T {
        self.value.get_or_insert_with(Default::default)
    }
}

#[test]
fn test_deref_mut_initializes_value() {
    struct TestStruct;

    impl Default for TestStruct {
        fn default() -> Self {
            TestStruct
        }
    }

    let mut lazy = Lazy { value: None };
    let value_ref = lazy.force_mut();
    assert!(value_ref.is::<TestStruct>());
}

#[test]
fn test_deref_mut_accesses_existing_value() {
    struct TestStruct;

    impl Default for TestStruct {
        fn default() -> Self {
            TestStruct
        }
    }

    let mut lazy = Lazy { value: Some(TestStruct) };
    let value_ref = lazy.force_mut();
    assert!(value_ref.is::<TestStruct>());
}

