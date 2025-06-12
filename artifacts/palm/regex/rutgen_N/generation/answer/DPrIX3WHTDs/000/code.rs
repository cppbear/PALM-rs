// Answer 0

#[test]
fn test_unwrap_class_bytes_success() {
    struct HirFrame {
        bytes: Vec<u8>,
    }

    impl HirFrame {
        fn unwrap_class_bytes(self) -> HirFrame {
            match self {
                HirFrame { bytes } => HirFrame { bytes },
                _ => panic!("tried to unwrap byte class from HirFrame, got: {:?}", self),
            }
        }
    }

    let frame = HirFrame { bytes: vec![1, 2, 3] };
    let unwrapped = frame.unwrap_class_bytes();
    assert_eq!(unwrapped.bytes, vec![1, 2, 3]);
}

#[test]
#[should_panic(expected = "tried to unwrap byte class from HirFrame")]
fn test_unwrap_class_bytes_failure() {
    struct HirFrame {
        other_data: String,
    }

    impl HirFrame {
        fn unwrap_class_bytes(self) -> HirFrame {
            match self {
                HirFrame { .. } => panic!("tried to unwrap byte class from HirFrame, got: {:?}", self),
            }
        }
    }

    let frame = HirFrame { other_data: String::from("not a byte class") };
    frame.unwrap_class_bytes();
}

