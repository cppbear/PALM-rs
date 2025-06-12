// Answer 0

#[test]
fn test_as_str_empty_string() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![String::from("")]
        }),
        cache: CachedThreadLocal::new(),
    };
    let regex = Regex(exec);
    let _result = regex.as_str();
}

#[test]
fn test_as_str_non_empty_ascii_string() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![String::from("hello")],
        }),
        cache: CachedThreadLocal::new(),
    };
    let regex = Regex(exec);
    let _result = regex.as_str();
}

#[test]
fn test_as_str_non_empty_unicode_string() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![String::from("こんにちは")],
        }),
        cache: CachedThreadLocal::new(),
    };
    let regex = Regex(exec);
    let _result = regex.as_str();
}

#[test]
fn test_as_str_very_long_string() {
    let long_string = "a".repeat(1_000_000); // 1 million characters
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![long_string],
        }),
        cache: CachedThreadLocal::new(),
    };
    let regex = Regex(exec);
    let _result = regex.as_str();
}

