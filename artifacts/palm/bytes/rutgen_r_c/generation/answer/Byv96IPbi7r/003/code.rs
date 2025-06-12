// Answer 0

#[test]
fn test_owned_drop_impl_underflow() {
    // Arrange
    struct MockDrop {
        drop_called: bool,
    }

    impl MockDrop {
        fn new() -> Self {
            Self { drop_called: false }
        }

        unsafe fn mock_drop_fn(_owned: *mut ()) {
            // Simulate the behavior of the drop function being called
        }
    }

    let mut mock_drop = MockDrop::new();
    let ref_cnt = AtomicUsize::new(1); // Set up a ref count of 1
    
    let lifetime = OwnedLifetime {
        ref_cnt,
        drop: MockDrop::mock_drop_fn,
    };

    let owned: *mut () = &mut lifetime as *mut _ as *mut ();

    // Act
    unsafe {
        owned_drop_impl(owned);
    }

    // Assert
    assert_eq!(mock_drop.drop_called, false, "Drop should not be called when ref count goes down to zero.");
}

#[test]
#[should_panic(expected = "expected non-zero refcount and no underflow")]
fn test_owned_drop_impl_panic_on_underflow() {
    // Arrange
    struct MockDrop {
        drop_called: bool,
    }
    
    impl MockDrop {
        fn new() -> Self {
            Self { drop_called: false }
        }

        unsafe fn mock_drop_fn(_owned: *mut ()) {
            // Simulate the behavior of the drop function being called
            drop_called = true; // Records that the drop has been called
        }
    }

    let ref_cnt = AtomicUsize::new(0); // Set up a ref count of 0 to trigger the panic

    let lifetime = OwnedLifetime {
        ref_cnt,
        drop: MockDrop::mock_drop_fn,
    };

    let owned: *mut () = &mut lifetime as *mut _ as *mut ();

    // Act
    unsafe {
        // This call should panic due to old_cnt being 0
        owned_drop_impl(owned);
    }
}

