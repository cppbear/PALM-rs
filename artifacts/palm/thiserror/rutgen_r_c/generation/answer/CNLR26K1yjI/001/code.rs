// Answer 0

#[test]
fn test_as_display_with_valid_pathbuf() {
    use std::path::PathBuf;

    let path = PathBuf::from("/some/valid/path");
    let display = path.as_display();

    assert_eq!(display.to_string(), "/some/valid/path");
}

#[test]
fn test_as_display_with_empty_pathbuf() {
    use std::path::PathBuf;

    let path = PathBuf::from("");
    let display = path.as_display();

    assert_eq!(display.to_string(), "");
}

#[test]
fn test_as_display_with_current_directory() {
    use std::path::PathBuf;

    let path = PathBuf::from(".");
    let display = path.as_display();

    assert_eq!(display.to_string(), ".");
}

#[test]
fn test_as_display_with_parent_directory() {
    use std::path::PathBuf;

    let path = PathBuf::from("..");
    let display = path.as_display();

    assert_eq!(display.to_string(), "..");
}

#[test]
fn test_as_display_with_root_directory() {
    use std::path::PathBuf;

    let path = PathBuf::from("/");
    let display = path.as_display();

    assert_eq!(display.to_string(), "/");
}

