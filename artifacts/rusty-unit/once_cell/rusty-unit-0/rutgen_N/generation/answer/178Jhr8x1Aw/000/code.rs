// Answer 0

#[test]
fn test_try_insert_initial_empty() {
    struct MyCell {
        value: Option<i32>,
    }

    impl MyCell {
        fn new() -> Self {
            MyCell { value: None }
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }

        fn get_or_init<F>(&mut self, init: F) -> &i32 
        where 
            F: FnOnce() -> i32,
        {
            if self.value.is_none() {
                self.value = Some(init());
            }
            self.value.as_ref().unwrap()
        }

        fn try_insert(&mut self, value: i32) -> Result<&i32, (&i32, i32)> {
            let mut value = Some(value);
            let res = self.get_or_init(|| value.take().unwrap());
            match value {
                None => Ok(res),
                Some(value) => Err((res, value)),
            }
        }
    }

    let mut cell = MyCell::new();
    assert!(cell.get().is_none());

    assert_eq!(cell.try_insert(92), Ok(&92));
    assert_eq!(cell.try_insert(62), Err((&92, 62)));

    assert!(cell.get().is_some());
}

#[test]
fn test_try_insert_with_value() {
    struct MyCell {
        value: Option<i32>,
    }

    impl MyCell {
        fn new() -> Self {
            MyCell { value: None }
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }

        fn get_or_init<F>(&mut self, init: F) -> &i32 
        where 
            F: FnOnce() -> i32,
        {
            if self.value.is_none() {
                self.value = Some(init());
            }
            self.value.as_ref().unwrap()
        }

        fn try_insert(&mut self, value: i32) -> Result<&i32, (&i32, i32)> {
            let mut value = Some(value);
            let res = self.get_or_init(|| value.take().unwrap());
            match value {
                None => Ok(res),
                Some(value) => Err((res, value)),
            }
        }
    }

    let mut cell = MyCell::new();
    assert!(cell.get().is_none());

    assert_eq!(cell.try_insert(100), Ok(&100));
    assert_eq!(cell.try_insert(200), Err((&100, 200)));
    assert_eq!(cell.try_insert(150), Err((&100, 150)));
}

