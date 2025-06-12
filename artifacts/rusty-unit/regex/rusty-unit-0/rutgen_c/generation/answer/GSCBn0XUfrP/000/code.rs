// Answer 0

#[test]
fn test_regex_as_str() {
    struct DummyExecReadOnly {
        res: Vec<String>,
    }

    struct DummyExec {
        ro: Arc<DummyExecReadOnly>,
    }

    impl DummyExec {
        fn regex_strings(&self) -> &[String] {
            &self.ro.res
        }
    }

    struct Regex(DummyExec);

    let exec_read_only = DummyExecReadOnly {
        res: vec!["test_string".to_string()],
    };
    
    let exec = DummyExec {
        ro: Arc::new(exec_read_only),
    };
    
    let regex = Regex(exec);
    
    assert_eq!(regex.as_str(), "test_string");
}

#[test]
fn test_regex_as_str_empty() {
    struct DummyExecReadOnly {
        res: Vec<String>,
    }

    struct DummyExec {
        ro: Arc<DummyExecReadOnly>,
    }

    impl DummyExec {
        fn regex_strings(&self) -> &[String] {
            &self.ro.res
        }
    }

    struct Regex(DummyExec);

    let exec_read_only = DummyExecReadOnly {
        res: vec!["".to_string()],
    };
    
    let exec = DummyExec {
        ro: Arc::new(exec_read_only),
    };
    
    let regex = Regex(exec);
    
    assert_eq!(regex.as_str(), "");
}

