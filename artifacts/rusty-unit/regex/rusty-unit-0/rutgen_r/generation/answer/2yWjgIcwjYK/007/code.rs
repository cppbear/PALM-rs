// Answer 0

fn fmt_test() {
    use std::fmt;
    
    struct MockInst {
        goto: usize,
        ranges: Vec<(u8, u8)>,
    }

    struct MockSelf {
        instructions: Vec<MockInst>,
        start: usize,
    }

    impl MockSelf {
        fn iter(&self) -> std::slice::Iter<MockInst> {
            self.instructions.iter()
        }
    }

    impl fmt::Display for MockSelf {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use std::fmt;
            for (pc, inst) in self.iter().enumerate() {
                if let Some(ranges) = inst.ranges.get(0) {
                    let ranges_string = format!("{:?}-{:?}", ranges.0, ranges.1);
                    write!(f, "{:04} {}", pc, ranges_string)?;
                }
                if pc == self.start {
                    write!(f, " (start)")?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }

    #[test]
    fn test_fmt_with_ranged_inst() {
        let instructions = vec![
            MockInst { goto: 2, ranges: vec![(b'a', b'z')] },
            MockInst { goto: 1, ranges: vec![(b'0', b'9')] },
            MockInst { goto: 3, ranges: vec![] },
        ];

        let mock_self = MockSelf { instructions, start: 0 };
        let result = format!("{}", mock_self);
        assert_eq!(result, "000 97-122\n001 48-57\n002 \n (start)\n");
    }

    #[test]
    #[should_panic(expected = "failed to write to formatter")]
    fn test_fmt_panics_on_error() {
        struct BadMockSelf {
            instructions: Vec<MockInst>,
            start: usize,
        }

        impl fmt::Display for BadMockSelf {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                for (pc, _inst) in self.instructions.iter().enumerate() {
                    if pc > 1 { // force an error for demonstration via out of bounds 
                        write!(f, "{:04} Error", pc)?;
                    }
                }
                Ok(())
            }
        }

        let bad_mock_self = BadMockSelf {
            instructions: vec![
                MockInst { goto: 0, ranges: vec![(1, 2)] },
                MockInst { goto: 0, ranges: vec![(1, 2)] },
            ],
            start: 0,
        };

        let _ = format!("{}", bad_mock_self);
    }
}

