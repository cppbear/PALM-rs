// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;
    use hashbrown::HashSet;

    #[derive(Clone)]
    struct TestSet {
        data: HashSet<i32>,
    }

    impl fmt::Debug for TestSet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let mut set = TestSet {
        data: HashSet::new(),
    };
    set.data.insert(1);
    set.data.insert(2);
    set.data.insert(3);

    let expected_output = "[1, 2, 3]";
    let mut actual_output = String::new();
    {
        let f = &mut fmt::Formatter::new(&mut actual_output);
        let _ = set.fmt(f);
    }
    
    assert_eq!(actual_output.trim(), expected_output);
}

