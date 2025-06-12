// Answer 0

#[test]
fn test_capture_names_single_non_empty() {
    let capture_names_vec = vec![Some(String::from("first_capture"))];
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly { captures: capture_names_vec }), cache: CachedThreadLocal::new() });
    let _ = regex.capture_names();
}

#[test]
fn test_capture_names_multiple_non_empty() {
    let capture_names_vec = vec![Some(String::from("first_capture")), Some(String::from("second_capture")), Some(String::from("third_capture"))];
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly { captures: capture_names_vec }), cache: CachedThreadLocal::new() });
    let _ = regex.capture_names();
}

#[test]
fn test_capture_names_with_none() {
    let capture_names_vec = vec![Some(String::from("capture_one")), None, Some(String::from("capture_two"))];
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly { captures: capture_names_vec }), cache: CachedThreadLocal::new() });
    let _ = regex.capture_names();
}

#[test]
fn test_capture_names_empty_with_one_valid() {
    let capture_names_vec = vec![Some(String::from("only_capture"))];
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly { captures: capture_names_vec }), cache: CachedThreadLocal::new() });
    let _ = regex.capture_names();
}

#[test]
fn test_capture_names_zero_length() {
    let capture_names_vec: Vec<Option<String>> = Vec::new();
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly { captures: capture_names_vec }), cache: CachedThreadLocal::new() });
    let _ = regex.capture_names();
}

#[test]
fn test_capture_names_large_vector() {
    let capture_names_vec: Vec<Option<String>> = (0..20).map(|i| Some(format!("capture_{}", i))).collect();
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly { captures: capture_names_vec }), cache: CachedThreadLocal::new() });
    let _ = regex.capture_names();
}

