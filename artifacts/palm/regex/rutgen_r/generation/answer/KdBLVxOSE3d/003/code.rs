// Answer 0

#[derive(Debug)]
enum GroupKind {
    CaptureIndex(usize),
    CaptureName(String),
    NonCapturing(String),
}

struct Group {
    kind: GroupKind,
}

impl Group {
    pub fn is_capturing(&self) -> bool {
        match self.kind {
            GroupKind::CaptureIndex(_) | GroupKind::CaptureName(_) => true,
            GroupKind::NonCapturing(_) => false,
        }
    }
}

#[test]
fn test_is_capturing_capture_index() {
    let group = Group {
        kind: GroupKind::CaptureIndex(1),
    };
    assert_eq!(group.is_capturing(), true);
}

#[test]
fn test_is_capturing_capture_name() {
    let group = Group {
        kind: GroupKind::CaptureName("group1".to_string()),
    };
    assert_eq!(group.is_capturing(), true);
}

#[test]
fn test_is_not_capturing_non_capturing() {
    let group = Group {
        kind: GroupKind::NonCapturing("non_capturing_group".to_string()),
    };
    assert_eq!(group.is_capturing(), false);
}

