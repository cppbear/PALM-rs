// Answer 0

#[test]
fn test_as_display_for_path_buf() {
    use std::path::PathBuf;

    let path_buf = PathBuf::from("/some/path");
    let display = path_buf.as_display();
    
    assert_eq!(display.to_string(), "/some/path");
}

#[test]
fn test_as_display_for_empty_path_buf() {
    use std::path::PathBuf;

    let path_buf = PathBuf::from("");
    let display = path_buf.as_display();
    
    assert_eq!(display.to_string(), "");
}

