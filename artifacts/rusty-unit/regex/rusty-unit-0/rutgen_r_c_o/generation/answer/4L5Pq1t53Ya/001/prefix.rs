// Answer 0

#[test]
fn test_capture_names_non_empty() {
    let capture_names = vec![Some("name1".to_string()), Some("name2".to_string())];
    let exec_read_only = ExecReadOnly { captures: capture_names.clone() };
    let exec = Exec { ro: Arc::new(exec_read_only), cache: CachedThreadLocal::new() };
    let regex = Regex(exec);

    let names = regex.capture_names();
}

#[test]
fn test_capture_names_single() {
    let capture_names = vec![Some("single_name".to_string())];
    let exec_read_only = ExecReadOnly { captures: capture_names.clone() };
    let exec = Exec { ro: Arc::new(exec_read_only), cache: CachedThreadLocal::new() };
    let regex = Regex(exec);

    let names = regex.capture_names();
}

#[test]
fn test_capture_names_empty() {
    let capture_names: Vec<Option<String>> = vec![];
    let exec_read_only = ExecReadOnly { captures: capture_names.clone() };
    let exec = Exec { ro: Arc::new(exec_read_only), cache: CachedThreadLocal::new() };
    let regex = Regex(exec);

    let names = regex.capture_names();
}

#[test]
fn test_capture_names_with_none() {
    let capture_names = vec![Some("name1".to_string()), None];
    let exec_read_only = ExecReadOnly { captures: capture_names.clone() };
    let exec = Exec { ro: Arc::new(exec_read_only), cache: CachedThreadLocal::new() };
    let regex = Regex(exec);

    let names = regex.capture_names();
}

#[test]
fn test_capture_names_unique_names() {
    let capture_names = vec![Some("name1".to_string()), Some("name2".to_string()), Some("name3".to_string())];
    let exec_read_only = ExecReadOnly { captures: capture_names.clone() };
    let exec = Exec { ro: Arc::new(exec_read_only), cache: CachedThreadLocal::new() };
    let regex = Regex(exec);

    let names = regex.capture_names();
}

#[test]
fn test_capture_names_longer_names() {
    let capture_names = vec![Some("this_is_a_long_capture_name".to_string())];
    let exec_read_only = ExecReadOnly { captures: capture_names.clone() };
    let exec = Exec { ro: Arc::new(exec_read_only), cache: CachedThreadLocal::new() };
    let regex = Regex(exec);

    let names = regex.capture_names();
}

#[test]
fn test_capture_names_mixed_cases() {
    let capture_names = vec![Some("Name1".to_string()), Some("name2".to_string()), Some("NAME3".to_string())];
    let exec_read_only = ExecReadOnly { captures: capture_names.clone() };
    let exec = Exec { ro: Arc::new(exec_read_only), cache: CachedThreadLocal::new() };
    let regex = Regex(exec);

    let names = regex.capture_names();
}

