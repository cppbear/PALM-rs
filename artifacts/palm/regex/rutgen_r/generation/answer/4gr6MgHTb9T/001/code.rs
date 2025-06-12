// Answer 0

#[test]
fn test_compile_finish_success() {
    struct MockInst {
        value: Option<u32>,
    }

    struct MockByteClasses {
        classes: Vec<u32>,
    }

    struct MockCompiled {
        insts: Vec<u32>,
        byte_classes: Vec<u32>,
        capture_name_idx: Arc<Vec<String>>,
    }

    struct MockCompile {
        insts: Vec<Result<MockInst, ()>>,
        byte_classes: MockByteClasses,
        compiled: MockCompiled,
        capture_name_idx: Vec<String>,
    }

    impl MockCompile {
        fn compile_finish(mut self) -> result::Result<MockCompiled, ()> {
            self.compiled.insts =
                self.insts.into_iter().map(|inst| inst.unwrap().value).collect();
            self.compiled.byte_classes = self.byte_classes.classes.clone();
            self.compiled.capture_name_idx = Arc::new(self.capture_name_idx);
            Ok(self.compiled)
        }
    }

    let insts = vec![Ok(MockInst { value: Some(1) }),
                     Ok(MockInst { value: Some(2) }),
                     Ok(MockInst { value: Some(3) })];
    let byte_classes = MockByteClasses { classes: vec![5, 6] };
    let capture_name_idx = vec!["group1".to_string(), "group2".to_string()];

    let compiled = MockCompiled {
        insts: vec![],
        byte_classes: vec![],
        capture_name_idx: Arc::new(vec![]),
    };

    let mock_compile = MockCompile {
        insts,
        byte_classes,
        compiled,
        capture_name_idx,
    };

    let result = mock_compile.compile_finish();
    assert!(result.is_ok());
    let compiled_result = result.unwrap();

    assert_eq!(compiled_result.insts, vec![Some(1), Some(2), Some(3)]);
    assert_eq!(compiled_result.byte_classes, vec![5, 6]);
    assert_eq!(*compiled_result.capture_name_idx, vec!["group1".to_string(), "group2".to_string()]);
}

