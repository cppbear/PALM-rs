// Answer 0

#[test]
fn test_capture_index_non_capturing_group() {
    struct NonCapturingGroup;

    enum GroupKind {
        CaptureIndex(u32),
        CaptureName(CaptureName),
        NonCapturing(NonCapturingGroup),
    }

    struct CaptureName {
        index: u32,
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

    let group = Group {
        kind: GroupKind::NonCapturing(NonCapturingGroup),
    };
    
    assert_eq!(group.capture_index(), None);
}

