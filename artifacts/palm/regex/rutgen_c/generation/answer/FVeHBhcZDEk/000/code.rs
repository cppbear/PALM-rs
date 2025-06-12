// Answer 0

#[test]
fn test_c_byte_with_valid_byte() {
    struct FakeHir;
    
    impl hir::Hir for FakeHir {}

    struct FakeCompiler {
        insts: Vec<MaybeInst>,
        compiled: Program,
        byte_classes: ByteClassSet,
    }

    impl Compiler for FakeCompiler {
        fn c_class_bytes(&mut self, _ranges: &[hir::ClassBytesRange]) -> Result {
            Ok(Patch {
                hole: Hole::None,
                entry: InstPtr::new(0),
            })
        }
    }

    let mut compiler = Compiler::new();
    compiler.c_byte(0x61).unwrap(); // 0x61 is 'a'
}

#[test]
fn test_c_byte_with_upper_limit() {
    struct FakeHir;

    impl hir::Hir for FakeHir {}

    struct FakeCompiler {
        insts: Vec<MaybeInst>,
        compiled: Program,
        byte_classes: ByteClassSet,
    }

    impl Compiler for FakeCompiler {
        fn c_class_bytes(&mut self, _ranges: &[hir::ClassBytesRange]) -> Result {
            Ok(Patch {
                hole: Hole::None,
                entry: InstPtr::new(0),
            })
        }
    }

    let mut compiler = Compiler::new();
    compiler.c_byte(0xFF).unwrap(); // the upper limit for a byte
}

#[test]
#[should_panic]
fn test_c_byte_with_invalid_byte() {
    struct FakeHir;

    impl hir::Hir for FakeHir {}

    struct FakeCompiler {
        insts: Vec<MaybeInst>,
        compiled: Program,
        byte_classes: ByteClassSet,
    }

    impl Compiler for FakeCompiler {
        fn c_class_bytes(&mut self, _ranges: &[hir::ClassBytesRange]) -> Result {
            Err(Error::CompiledTooBig(10))
        }
    }

    let mut compiler = Compiler::new();
    compiler.c_byte(0x100); // this should panic since 0x100 is out of byte range
}

