// Answer 0

#[test]
fn test_thread_rng_fmt() {
    struct MockCore;
    struct MockOsRng;

    impl BlockRngCore for MockCore {}
    impl SeedableRng for MockCore {}
    impl TryRngCore for MockOsRng {}

    let reseeding_rng = ReseedingRng(MockCore, MockOsRng);
    let thread_rng = ThreadRng {
        rng: Rc::new(UnsafeCell::new(reseeding_rng)),
    };

    let mut output = String::new();
    let result = thread_rng.fmt(&mut fmt::Formatter::new(&mut output));

    assert!(result.is_ok());
    assert_eq!(output, "ThreadRng {{ .. }}");
}

