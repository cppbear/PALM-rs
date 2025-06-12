// Answer 0

#[test]
fn test_cmd_ast_valid_pattern() {
    struct Args {
        arg_pattern: String,
    }

    let args = Args {
        arg_pattern: String::from(r"^[a-zA-Z]+$"),
    };

    let result = cmd_ast(&args);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_cmd_ast_invalid_pattern() {
    struct Args {
        arg_pattern: String,
    }

    let args = Args {
        arg_pattern: String::from(r"["), // Invalid regex pattern
    };

    let _ = cmd_ast(&args).unwrap(); // This should panic
}

