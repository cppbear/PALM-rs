// Answer 0

#[test]
fn test_cmd_hir_valid_pattern() {
    use std::collections::HashMap;
    use std::env;

    struct Args {
        arg_pattern: String,
    }

    let args = Args {
        arg_pattern: String::from("a*b"),
    };

    let result = cmd_hir(&args);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "invalid regex pattern")]
fn test_cmd_hir_invalid_pattern() {
    struct Args {
        arg_pattern: String,
    }

    let args = Args {
        arg_pattern: String::from("["),
    };

    cmd_hir(&args).unwrap();
}

#[test]
fn test_cmd_hir_empty_pattern() {
    struct Args {
        arg_pattern: String,
    }

    let args = Args {
        arg_pattern: String::from(""),
    };

    let result = cmd_hir(&args);
    assert!(result.is_ok());
}

