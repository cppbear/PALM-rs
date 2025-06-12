// Answer 0

#[test]
fn test_capture_index_with_capture_name() {
    struct CaptureName {
        index: u32,
    }

    enum GroupKind {
        CaptureIndex(u32),
        CaptureName(CaptureName),
        NonCapturing(u32),
    }

    struct Group {
        kind: GroupKind,
    }

    impl Group {
        pub fn capture_index(&self) -> Option<u32> {
            match self.kind {
                GroupKind::CaptureIndex(i) => Some(i),
                GroupKind::CaptureName(ref x) => Some(x.index),
                GroupKind::NonCapturing(_) => None,
            }
        }
    }

    let capture_name = CaptureName { index: 42 };
    let group = Group {
        kind: GroupKind::CaptureName(capture_name),
    };

    assert_eq!(group.capture_index(), Some(42));
}

#[test]
fn test_capture_index_with_another_capture_name() {
    struct CaptureName {
        index: u32,
    }

    enum GroupKind {
        CaptureIndex(u32),
        CaptureName(CaptureName),
        NonCapturing(u32),
    }

    struct Group {
        kind: GroupKind,
    }

    impl Group {
        pub fn capture_index(&self) -> Option<u32> {
            match self.kind {
                GroupKind::CaptureIndex(i) => Some(i),
                GroupKind::CaptureName(ref x) => Some(x.index),
                GroupKind::NonCapturing(_) => None,
            }
        }
    }

    let capture_name = CaptureName { index: 100 };
    let group = Group {
        kind: GroupKind::CaptureName(capture_name),
    };

    assert_eq!(group.capture_index(), Some(100));
}

