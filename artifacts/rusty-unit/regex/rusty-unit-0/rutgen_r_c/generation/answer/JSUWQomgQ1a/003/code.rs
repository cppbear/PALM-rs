// Answer 0

#[test]
fn test_cmd_utf8_ranges_valid_class() {
    struct TestArgs {
        arg_class: String,
    }
    
    let args = TestArgs {
        arg_class: "a-zA".to_string(),
    };
    
    let result = cmd_utf8_ranges(&args);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_utf8_ranges_invalid_hir_class() {
    struct TestArgs {
        arg_class: String,
    }
    
    let args = TestArgs {
        arg_class: "123".to_string(),
    };
    
    let result = cmd_utf8_ranges(&args);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "unexpected HIR, expected Unicode class"
    );
}

#[test]
fn test_cmd_utf8_ranges_another_invalid_hir() {
    struct TestArgs {
        arg_class: String,
    }
    
    let args = TestArgs {
        arg_class: "!!!".to_string(),
    };
    
    let result = cmd_utf8_ranges(&args);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "unexpected HIR, expected Unicode class"
    );
}

#[test]
fn test_cmd_utf8_ranges_empty_class() {
    struct TestArgs {
        arg_class: String,
    }
    
    let args = TestArgs {
        arg_class: "".to_string(),
    };
    
    let result = cmd_utf8_ranges(&args);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "unexpected HIR, expected Unicode class"
    );
}

