// Answer 0

#[test]
fn test_fmt() {
    struct ThreadRng;

    impl std::fmt::Debug for ThreadRng {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(fmt, "ThreadRng {{ .. }}")
        }
    }

    let rng = ThreadRng;
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", rng);
    assert!(result.is_ok());
    assert_eq!(output, "ThreadRng { .. }");
}

