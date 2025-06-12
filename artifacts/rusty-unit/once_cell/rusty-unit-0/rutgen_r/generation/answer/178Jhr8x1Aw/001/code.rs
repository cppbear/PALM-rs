// Answer 0

#[test]
fn test_try_insert_with_conflict() {
    struct OnceCell<T> {
        value: Option<T>,
    }

    impl<T> OnceCell<T> {
        pub fn new() -> Self {
            OnceCell { value: None }
        }

        pub fn get(&self) -> Option<&T> {
            self.value.as_ref()
        }

        pub fn try_insert(&mut self, value: T) -> Result<&T, (&T, T)> {
            if let Some(ref existing) = self.value {
                return Err((existing, value));
            }
            self.value = Some(value);
            Ok(self.value.as_ref().unwrap())
        }
    }

    let mut cell = OnceCell::new();
    assert!(cell.get().is_none());

    assert_eq!(cell.try_insert(92), Ok(&92));
    assert_eq!(cell.try_insert(62), Err((&92, 62)));

    assert!(cell.get().is_some());
}

