// Answer 0

#[test]
fn test_source_output_slice_too_small() {
    struct DecodeSliceError {
        kind: ErrorKind,
    }
    
    enum ErrorKind {
        DecodeError(Box<dyn std::error::Error + 'static>),
        OutputSliceTooSmall,
    }
    
    impl DecodeSliceError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self.kind {
                ErrorKind::DecodeError(ref e) => Some(e),
                ErrorKind::OutputSliceTooSmall => None,
            }
        }
    }

    let error = DecodeSliceError {
        kind: ErrorKind::OutputSliceTooSmall,
    };
    
    assert_eq!(error.source(), None);
}

