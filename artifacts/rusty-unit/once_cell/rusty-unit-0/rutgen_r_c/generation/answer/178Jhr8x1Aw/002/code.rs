// Answer 0

#[test]
fn test_try_insert_success() {
    struct TestCell(Imp<i32>);

    impl TestCell {
        const fn new() -> Self {
            TestCell(Imp::new())
        }

        fn get(&self) -> Option<&i32> {
            self.0.get()
        }

        fn try_insert(&self, value: i32) -> Result<&i32, (&i32, i32)> {
            let mut value = Some(value);
            let res = self.get_or_init(|| unsafe { value.take().unwrap_unchecked() });
            match value {
                None => Ok(res),
                Some(value) => Err((res, value)),
            }
        }

        fn get_or_init<F>(&self, f: F) -> &i32
        where
            F: FnOnce() -> i32,
        {
            self.0.get_or_init(f)
        }
    }

    let cell = TestCell::new();
    assert!(cell.get().is_none());

    assert_eq!(cell.try_insert(42), Ok(&42));
    assert!(cell.get().is_some());
}

#[test]
fn test_try_insert_failure() {
    struct TestCell(Imp<i32>);

    impl TestCell {
        const fn new() -> Self {
            TestCell(Imp::new())
        }

        fn get(&self) -> Option<&i32> {
            self.0.get()
        }

        fn try_insert(&self, value: i32) -> Result<&i32, (&i32, i32)> {
            let mut value = Some(value);
            let res = self.get_or_init(|| unsafe { value.take().unwrap_unchecked() });
            match value {
                None => Ok(res),
                Some(value) => Err((res, value)),
            }
        }

        fn get_or_init<F>(&self, f: F) -> &i32
        where
            F: FnOnce() -> i32,
        {
            self.0.get_or_init(f)
        }
    }

    let cell = TestCell::new();
    assert!(cell.get().is_none());

    assert_eq!(cell.try_insert(99), Ok(&99));
    assert_eq!(cell.try_insert(77), Err((&99, 77)));
    assert!(cell.get().is_some());
}

