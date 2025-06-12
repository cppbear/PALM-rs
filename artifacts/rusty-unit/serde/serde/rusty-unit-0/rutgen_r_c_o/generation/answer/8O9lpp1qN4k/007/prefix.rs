// Answer 0

#[test]
fn test_unexpected_bytes_empty() {
    let content = Content::Bytes(vec![]);
    content.unexpected();
}

#[test]
fn test_unexpected_bytes_single_byte() {
    let content = Content::Bytes(vec![1]);
    content.unexpected();
}

#[test]
fn test_unexpected_bytes_two_bytes() {
    let content = Content::Bytes(vec![1, 2]);
    content.unexpected();
}

#[test]
fn test_unexpected_bytes_max_length() {
    let content = Content::Bytes(vec![0; 1024]);
    content.unexpected();
}

#[test]
fn test_unexpected_bytes_varied_length() {
    let lengths = [3, 7, 15, 50, 100, 200, 512, 1023];
    for &length in &lengths {
        let content = Content::Bytes(vec![0; length]);
        content.unexpected();
    }
}

