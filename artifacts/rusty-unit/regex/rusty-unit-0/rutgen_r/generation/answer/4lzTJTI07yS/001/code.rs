// Answer 0

#[test]
fn test_c_with_empty_expression() {
    struct MockCompiler {
        insts: Vec<MaybeInst>,
        compiled: Compiled,
        byte_classes: ByteClasses,
    }

    impl MockCompiler {
        fn check_size(&mut self) -> Result<()> {
            Ok(())
        }

        fn c_literal(&mut self, _chars: &[char]) -> Result<Patch> {
            self.insts.push(MaybeInst::Literal);
            Ok(Patch { hole: Hole::None, entry: self.insts.len() - 1 })
        }

        fn c_empty_look(&mut self, _look: prog::EmptyLook) -> Result<Patch> {
            Ok(Patch { hole: Hole::None, entry: self.insts.len() })
        }
    }

    let mut compiler = MockCompiler {
        insts: Vec::new(),
        compiled: Compiled::default(),
        byte_classes: ByteClasses::default(),
    };

    let expr = Hir::new(Empty);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert_eq!(patch.entry, 0);
}

#[test]
fn test_c_with_literal_unicode() {
    struct MockCompiler {
        insts: Vec<MaybeInst>,
        compiled: Compiled,
        byte_classes: ByteClasses,
    }

    impl MockCompiler {
        fn check_size(&mut self) -> Result<()> {
            Ok(())
        }

        fn c_literal(&mut self, _chars: &[char]) -> Result<Patch> {
            self.insts.push(MaybeInst::Literal);
            Ok(Patch { hole: Hole::None, entry: self.insts.len() - 1 })
        }
    }

    let mut compiler = MockCompiler {
        insts: Vec::new(),
        compiled: Compiled::default(),
        byte_classes: ByteClasses::default(),
    };

    let expr = Hir::new(Literal(hir::Literal::Unicode('a')));
    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert_eq!(patch.entry, 0);
}

#[should_panic]
#[test]
fn test_c_panic_on_check_size_error() {
    struct MockCompiler {
        insts: Vec<MaybeInst>,
        compiled: Compiled,
        byte_classes: ByteClasses,
    }

    impl MockCompiler {
        fn check_size(&mut self) -> Result<()> {
            Err(Error::OutOfMemory)
        }

        fn c_literal(&mut self, _: &[char]) -> Result<Patch> {
            Ok(Patch { hole: Hole::None, entry: 0 })
        }
    }

    let mut compiler = MockCompiler {
        insts: Vec::new(),
        compiled: Compiled::default(),
        byte_classes: ByteClasses::default(),
    };

    let expr = Hir::new(Literal(hir::Literal::Unicode('a')));
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_with_class_unicode() {
    struct MockCompiler {
        insts: Vec<MaybeInst>,
        compiled: Compiled,
        byte_classes: ByteClasses,
    }

    impl MockCompiler {
        fn check_size(&mut self) -> Result<()> {
            Ok(())
        }

        fn c_class(&mut self, _ranges: &[ClassUnicodeRange]) -> Result<Patch> {
            self.insts.push(MaybeInst::Class);
            Ok(Patch { hole: Hole::None, entry: self.insts.len() - 1 })
        }
    }

    let mut compiler = MockCompiler {
        insts: Vec::new(),
        compiled: Compiled::default(),
        byte_classes: ByteClasses::default(),
    };

    let expr = Hir::new(Class(hir::Class::Unicode(vec![ClassUnicodeRange::new('a', 'z')])));
    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert_eq!(patch.entry, 0);
}

