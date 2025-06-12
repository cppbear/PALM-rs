// Answer 0

#[test]
fn test_fmt() {
    struct DummyCaptures;

    impl std::fmt::Debug for DummyCaptures {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple("DummyCaptures").finish()
        }
    }

    let captures = DummyCaptures;
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        captures.fmt(&mut formatter).unwrap();
    }
    assert_eq!(output, "DummyCaptures");
}

