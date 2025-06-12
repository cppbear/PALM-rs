// Answer 0

#[test]
fn test_lazy_new_with_valid_function() {
    struct TestFunction;

    impl FnOnce<()> for TestFunction {
        type Output = i32;

        extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
            42
        }
    }

    let f = TestFunction;
    let lazy_value = new(f);
    assert!(lazy_value.cell.get().is_none());
}

#[test]
#[should_panic]
fn test_lazy_new_with_invalid_function() {
    struct InvalidFunction;

    impl FnOnce<()> for InvalidFunction {
        type Output = !;

        extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
            panic!("This is a panic from the function");
        }
    }

    let f = InvalidFunction;
    let lazy_value = new(f);
    assert!(lazy_value.cell.get().is_none());
}

