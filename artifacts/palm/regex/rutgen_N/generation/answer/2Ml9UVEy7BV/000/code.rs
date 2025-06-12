// Answer 0

#[test]
fn test_fmt() {
    struct TestState {
        flags_value: u32,
        inst_ptrs_value: Vec<usize>,
    }

    impl TestState {
        fn flags(&self) -> u32 {
            self.flags_value
        }

        fn inst_ptrs(&self) -> std::slice::Iter<usize> {
            self.inst_ptrs_value.iter()
        }
    }

    let state = TestState {
        flags_value: 5,
        inst_ptrs_value: vec![1, 2, 3],
    };

    let mut output = vec![];
    let result = std::fmt::write(&mut output, |f| state.fmt(f));

    assert!(result.is_ok());
    let expected_output = "State { flags: 5, insts: [1, 2, 3] }";
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

