// Answer 0

#[test]
fn test_capture_index_with_capture_index() {
    struct TestGroup {
        kind: GroupKind,
    }
    
    enum GroupKind {
        CaptureIndex(u32),
        CaptureName(CaptureName),
        NonCapturing(u32),
    }

    struct CaptureName {
        index: u32,
    }
    
    let group = TestGroup {
        kind: GroupKind::CaptureIndex(5),
    };

    assert_eq!(group.capture_index(), Some(5));
}

#[test]
fn test_capture_index_with_capture_name() {
    struct TestGroup {
        kind: GroupKind,
    }
    
    enum GroupKind {
        CaptureIndex(u32),
        CaptureName(CaptureName),
        NonCapturing(u32),
    }

    struct CaptureName {
        index: u32,
    }
    
    let group = TestGroup {
        kind: GroupKind::CaptureName(CaptureName { index: 7 }),
    };

    assert_eq!(group.capture_index(), Some(7));
}

#[test]
fn test_capture_index_with_non_capturing() {
    struct TestGroup {
        kind: GroupKind,
    }
    
    enum GroupKind {
        CaptureIndex(u32),
        CaptureName(CaptureName),
        NonCapturing(u32),
    }

    struct CaptureName {
        index: u32,
    }
    
    let group = TestGroup {
        kind: GroupKind::NonCapturing(3),
    };

    assert_eq!(group.capture_index(), None);
}

