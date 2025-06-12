// Answer 0

#[test]
fn test_once_cell_uninitialized() {
    struct OnceCell {
        value: Option<i32>,
    }

    impl OnceCell {
        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }
    }

    use std::fmt;

    impl fmt::Debug for OnceCell {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.get() {
                Some(v) => f.debug_tuple("OnceCell").field(v).finish(),
                None => f.write_str("OnceCell(Uninit)"),
            }
        }
    }

    let cell = OnceCell { value: None };
    let result = format!("{:?}", cell);
    assert_eq!(result, "OnceCell(Uninit)");
}

