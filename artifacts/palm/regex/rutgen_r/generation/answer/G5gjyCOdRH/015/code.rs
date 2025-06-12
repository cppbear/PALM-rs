// Answer 0

#[test]
fn test_visit_class_set_item_post_with_unicode_range() {
    struct Flags {
        unicode: bool,
    }

    impl Flags {
        fn unicode(&self) -> bool {
            self.unicode
        }
    }

    struct MockHir {
        flags: Flags,
        frames: Vec<HirFrame>,
    }

    impl MockHir {
        fn new(flags: Flags) -> Self {
            MockHir {
                flags,
                frames: vec![],
            }
        }

        fn flags(&self) -> &Flags {
            &self.flags
        }

        fn pop(&mut self) -> Option<Option<Class>> {
            self.frames.pop().map(|frame| match frame {
                HirFrame::ClassUnicode(cls) => Some(cls),
                HirFrame::ClassBytes(_cls) => None,
            })
        }

        fn push(&mut self, frame: HirFrame) {
            self.frames.push(frame);
        }

        fn unwrap_class_unicode(&mut self) -> ClassUnicode {
            ClassUnicode::new()
        }
    }

    struct ClassUnicode {
        ranges: Vec<ClassUnicodeRange>,
    }

    impl ClassUnicode {
        fn new() -> Self {
            ClassUnicode { ranges: vec![] }
        }

        fn push(&mut self, range: ClassUnicodeRange) {
            self.ranges.push(range);
        }
    }

    struct ClassUnicodeRange {
        start: char,
        end: char,
    }

    impl ClassUnicodeRange {
        fn new(start: char, end: char) -> Self {
            ClassUnicodeRange { start, end }
        }
    }

    struct HirFrame {
        frame_type: FrameType,
    }

    enum FrameType {
        ClassUnicode(ClassUnicode),
        ClassBytes(ClassBytes),
    }

    struct ClassBytes;

    let mut mock_hir = MockHir::new(Flags { unicode: true });
    
    mock_hir.push(HirFrame { frame_type: FrameType::ClassUnicode(mock_hir.unwrap_class_unicode()) });

    let ast_item = ast::ClassSetItem::Range(Box::new(ast::Range { start: ast::Literal { c: 'a' }, end: ast::Literal { c: 'z' } }));

    let result = mock_hir.visit_class_set_item_post(&ast_item);
    
    assert!(result.is_ok());
}

