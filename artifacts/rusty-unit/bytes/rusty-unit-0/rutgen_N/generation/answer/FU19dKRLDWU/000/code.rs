// Answer 0

#[derive(Debug)]
struct Buffer {
    ref_count: std::sync::Arc<std::sync::AtomicUsize>,
}

impl Buffer {
    fn new() -> Self {
        Buffer {
            ref_count: std::sync::Arc::new(std::sync::AtomicUsize::new(1)),
        }
    }

    fn is_unique(&self) -> bool {
        self.ref_count.load(std::sync::atomic::Ordering::Acquire) == 1
    }
}

#[test]
fn test_is_unique_when_ref_count_is_one() {
    let buffer = Buffer::new();
    assert!(buffer.is_unique());
}

#[test]
fn test_is_unique_when_ref_count_is_greater_than_one() {
    let buffer = Buffer::new();
    buffer.ref_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    assert!(!buffer.is_unique());
}

#[test]
fn test_is_unique_with_multiple_references() {
    let buffer = Buffer::new();
    let buffer_clone = buffer.ref_count.clone();
    buffer_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    assert!(!buffer.is_unique());
}

