// Answer 0

#[derive(Debug)]
struct Group {
    kind: GroupKind,
}

#[derive(Debug)]
enum GroupKind {
    CaptureIndex(usize),
    CaptureName(String),
    NonCapturing(String),
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
fn test_is_capturing_with_capture_index() {
    let group = Group {
        kind: GroupKind::CaptureIndex(1),
    };
    assert!(group.is_capturing());
}

#[test]
fn test_is_capturing_with_capture_name() {
    let group = Group {
        kind: GroupKind::CaptureName(String::from("name")),
    };
    assert!(group.is_capturing());
}

#[test]
fn test_is_capturing_with_non_capturing() {
    let group = Group {
        kind: GroupKind::NonCapturing(String::from("non_capturing")),
    };
    assert!(!group.is_capturing());
}

