// Answer 0

#[test]
fn test_regex_display_valid() {
    struct ExecReadOnly {
        regex_strings: Vec<String>,
    }
    
    let exec_read_only = ExecReadOnly { 
        regex_strings: vec![String::from("abc"), String::from("def")] 
    };
    let exec = Exec { 
        ro: Arc::new(exec_read_only), 
        cache: CachedThreadLocal::new() 
    };
    let regex = Regex(exec);

    let mut output = String::new();
    let result = regex.fmt(&mut fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert_eq!(output, "abc");
}

#[test]
#[should_panic]
fn test_regex_display_empty() {
    struct ExecReadOnly {
        regex_strings: Vec<String>,
    }

    let exec_read_only = ExecReadOnly { 
        regex_strings: vec![] 
    };
    let exec = Exec { 
        ro: Arc::new(exec_read_only), 
        cache: CachedThreadLocal::new() 
    };
    let regex = Regex(exec);

    let mut output = String::new();
    let _ = regex.fmt(&mut fmt::Formatter::new(&mut output)); // Should panic due to empty regex_strings
}

#[test]
fn test_regex_display_single_string() {
    struct ExecReadOnly {
        regex_strings: Vec<String>,
    }

    let exec_read_only = ExecReadOnly { 
        regex_strings: vec![String::from("xyz")] 
    };
    let exec = Exec { 
        ro: Arc::new(exec_read_only), 
        cache: CachedThreadLocal::new() 
    };
    let regex = Regex(exec);

    let mut output = String::new();
    let result = regex.fmt(&mut fmt::Formatter::new(&mut output));

    assert!(result.is_ok());
    assert_eq!(output, "xyz");
}

#[test]
fn test_regex_display_multiline_string() {
    struct ExecReadOnly {
        regex_strings: Vec<String>,
    }

    let exec_read_only = ExecReadOnly { 
        regex_strings: vec![String::from("one\ntwo\nthree")] 
    };
    let exec = Exec { 
        ro: Arc::new(exec_read_only), 
        cache: CachedThreadLocal::new() 
    };
    let regex = Regex(exec);

    let mut output = String::new();
    let result = regex.fmt(&mut fmt::Formatter::new(&mut output));

    assert!(result.is_ok());
    assert_eq!(output, "one\ntwo\nthree");
}

