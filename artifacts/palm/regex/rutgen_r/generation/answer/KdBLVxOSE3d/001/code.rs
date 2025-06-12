// Answer 0

fn main() {
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
    fn test_is_capturing_non_capturing() {
        let group = Group {
            kind: GroupKind::NonCapturing("non_capturing_group".to_string()),
        };
        assert_eq!(group.is_capturing(), false);
    }
}

