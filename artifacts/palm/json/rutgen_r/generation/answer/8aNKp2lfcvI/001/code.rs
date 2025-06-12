// Answer 0

#[test]
fn test_set_failed_success() {
    struct R;

    impl R {
        fn set_failed<T>(_: &mut T, failed: &mut bool) {
            *failed = true;
        }
    }

    let mut failed = false;
    {
        let mut instance = ();
        set_failed(&mut instance, &mut failed);
    }
    assert!(failed);
}

#[test]
#[should_panic]
fn test_set_failed_panic() {
    struct R;

    impl R {
        fn set_failed<T>(_: &mut T, _: &mut bool) {
            panic!("This is a deliberate panic for testing.");
        }
    }

    let mut failed = false;
    {
        let mut instance = ();
        set_failed(&mut instance, &mut failed);
    }
}

