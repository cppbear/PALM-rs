// Answer 0

#[test]
fn test_drop_elements_no_items() {
    struct TestType {
        value: i32,
    }

    impl TestType {
        const NEEDS_DROP: bool = true;
    }

    let mut table = RawTableInner {
        bucket_mask: 0,
        ctrl: NonNull::dangling(),
        growth_left: 0,
        items: 0,
    };

    unsafe {
        table.drop_elements::<TestType>();
    }
}

#[test]
#[should_panic]
fn test_drop_elements_with_items() {
    struct PanicType {
        value: i32,
    }

    impl PanicType {
        const NEEDS_DROP: bool = true;
    }

    let mut table = RawTableInner {
        bucket_mask: 0,
        ctrl: NonNull::dangling(),
        growth_left: 0,
        items: 1,
    };

    unsafe {
        table.drop_elements::<PanicType>();
    }
}

