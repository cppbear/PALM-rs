// Answer 0

fn test_is_capturing_capture_name() {
    struct Group {
        kind: GroupKind,
    }

    enum GroupKind {
        CaptureIndex(usize),
        CaptureName(String),
        NonCapturing(String),
    }

    let group_with_capture_name = Group {
        kind: GroupKind::CaptureName("group_name".to_string()),
    };
    
    assert!(group_with_capture_name.is_capturing());
}

fn test_is_capturing_capture_index() {
    struct Group {
        kind: GroupKind,
    }

    enum GroupKind {
        CaptureIndex(usize),
        CaptureName(String),
        NonCapturing(String),
    }
    
    let group_with_capture_index = Group {
        kind: GroupKind::CaptureIndex(1),
    };
    
    assert!(group_with_capture_index.is_capturing());
}

fn test_is_capturing_non_capturing() {
    struct Group {
        kind: GroupKind,
    }

    enum GroupKind {
        CaptureIndex(usize),
        CaptureName(String),
        NonCapturing(String),
    }
    
    let group_with_non_capturing = Group {
        kind: GroupKind::NonCapturing("non_capturing".to_string()),
    };

    assert!(!group_with_non_capturing.is_capturing());
}

