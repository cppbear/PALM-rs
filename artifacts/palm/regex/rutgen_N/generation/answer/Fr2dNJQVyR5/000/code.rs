// Answer 0

#[derive(Debug)]
struct Group {
    kind: GroupKind,
}

#[derive(Debug)]
enum GroupKind {
    CaptureIndex(u32),
    CaptureName(CaptureName),
    NonCapturing(()),
}

#[derive(Debug)]
struct CaptureName {
    index: u32,
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

#[test]
fn test_capture_index_with_capture_index() {
    let group = Group {
        kind: GroupKind::CaptureIndex(1),
    };
    assert_eq!(group.capture_index(), Some(1));
}

#[test]
fn test_capture_index_with_capture_name() {
    let group = Group {
        kind: GroupKind::CaptureName(CaptureName { index: 2 }),
    };
    assert_eq!(group.capture_index(), Some(2));
}

#[test]
fn test_capture_index_with_non_capturing() {
    let group = Group {
        kind: GroupKind::NonCapturing(()),
    };
    assert_eq!(group.capture_index(), None);
}

#[test]
fn test_capture_index_with_another_capture_index() {
    let group = Group {
        kind: GroupKind::CaptureIndex(0),
    };
    assert_eq!(group.capture_index(), Some(0));
}

