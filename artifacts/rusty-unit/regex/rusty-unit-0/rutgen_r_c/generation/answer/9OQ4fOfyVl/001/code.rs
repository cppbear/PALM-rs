// Answer 0

#[test]
fn test_regex_strings_empty() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct Exec {
        ro: Arc<ExecReadOnly>,
    }

    #[derive(Debug)]
    struct ExecReadOnly {
        res: Vec<String>,
    }

    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![],
        }),
    };

    let result = exec.regex_strings();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_regex_strings_single_entry() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct Exec {
        ro: Arc<ExecReadOnly>,
    }

    #[derive(Debug)]
    struct ExecReadOnly {
        res: Vec<String>,
    }

    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![String::from("a*b")],
        }),
    };

    let result = exec.regex_strings();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "a*b");
}

#[test]
fn test_regex_strings_multiple_entries() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct Exec {
        ro: Arc<ExecReadOnly>,
    }

    #[derive(Debug)]
    struct ExecReadOnly {
        res: Vec<String>,
    }

    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![String::from("foo"), String::from("bar"), String::from("baz")],
        }),
    };

    let result = exec.regex_strings();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], "foo");
    assert_eq!(result[1], "bar");
    assert_eq!(result[2], "baz");
}

#[test]
fn test_regex_strings_whitespace() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct Exec {
        ro: Arc<ExecReadOnly>,
    }

    #[derive(Debug)]
    struct ExecReadOnly {
        res: Vec<String>,
    }

    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![String::from(" "), String::from("\t"), String::from("\n")],
        }),
    };

    let result = exec.regex_strings();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], " ");
    assert_eq!(result[1], "\t");
    assert_eq!(result[2], "\n");
}

#[test]
fn test_regex_strings_special_characters() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct Exec {
        ro: Arc<ExecReadOnly>,
    }

    #[derive(Debug)]
    struct ExecReadOnly {
        res: Vec<String>,
    }

    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![String::from("?"), String::from(".+"), String::from("^$")],
        }),
    };

    let result = exec.regex_strings();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], "?");
    assert_eq!(result[1], ".+");
    assert_eq!(result[2], "^$");
}

