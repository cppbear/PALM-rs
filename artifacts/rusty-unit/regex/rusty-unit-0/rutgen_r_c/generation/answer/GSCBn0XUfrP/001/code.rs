// Answer 0

#[test]
fn test_regex_as_str_basic() {
    struct ExecReadOnly {
        res: Vec<String>,
    }

    struct Exec {
        ro: Arc<ExecReadOnly>,
    }

    struct Regex(Exec);

    let regex_strings = vec![String::from("abc")];
    let exec_read_only = Arc::new(ExecReadOnly { res: regex_strings });
    let exec = Exec { ro: exec_read_only };
    let regex = Regex(exec);

    assert_eq!(regex.as_str(), "abc");
}

#[test]
fn test_regex_as_str_empty() {
    struct ExecReadOnly {
        res: Vec<String>,
    }

    struct Exec {
        ro: Arc<ExecReadOnly>,
    }

    struct Regex(Exec);

    let regex_strings = vec![String::from("")];
    let exec_read_only = Arc::new(ExecReadOnly { res: regex_strings });
    let exec = Exec { ro: exec_read_only };
    let regex = Regex(exec);

    assert_eq!(regex.as_str(), "");
}

#[test]
#[should_panic]
fn test_regex_as_str_no_strings() {
    struct ExecReadOnly {
        res: Vec<String>,
    }

    struct Exec {
        ro: Arc<ExecReadOnly>,
    }

    struct Regex(Exec);

    let exec_read_only = Arc::new(ExecReadOnly { res: Vec::new() });
    let exec = Exec { ro: exec_read_only };
    let regex = Regex(exec);

    // This should panic since there are no strings in the regex_strings slice
    let _ = regex.as_str();
}

