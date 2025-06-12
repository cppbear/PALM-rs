// Answer 0

#[test]
fn test_force_mut_with_lazy() {
    use once_cell::unsync::Lazy;
    use once_cell::unsync::OnceCell;

    struct TestInit;
    impl TestInit {
        fn new() -> Self {
            TestInit
        }

        fn run(&self) -> i32 {
            42
        }
    }

    let mut lazy = Lazy::new(|| TestInit::new().run());
    
    // Ensure that the cell is empty to satisfy the first condition
    assert!(lazy.cell.get_mut().is_none());

    // Force the evaluation of the lazy value
    let result = force_mut(&mut lazy);
    
    // Verify that the result is as expected and the value has been initialized
    assert_eq!(*result, 42);
    assert_eq!(lazy.cell.get_mut(), Some(&mut 42));
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_panics_when_poisoned() {
    use once_cell::unsync::Lazy;

    let mut lazy: Lazy<i32, fn() -> i32> = Lazy::new(|| panic!("Simulated poison"));
    
    // Attempting to force evaluation should panic
    let _ = force_mut(&mut lazy);
}

