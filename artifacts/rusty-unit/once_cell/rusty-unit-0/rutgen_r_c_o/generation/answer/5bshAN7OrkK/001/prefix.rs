// Answer 0

#[test]
fn test_once_box_new() {
    let once_box: OnceBox<u32> = OnceBox::new();
}

#[test]
fn test_once_box_new_with_different_type() {
    let once_box: OnceBox<String> = OnceBox::new();
}

#[test]
fn test_once_box_new_empty() {
    let once_box: OnceBox<()> = OnceBox::new();
}

