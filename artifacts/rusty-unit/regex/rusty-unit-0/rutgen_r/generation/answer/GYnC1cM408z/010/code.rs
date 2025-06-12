// Answer 0

fn test_compile_many_valid_cases() {
    struct TestStruct {
        compiled: Compiled,
    }

    impl TestStruct {
        fn needs_dotstar(&self) -> bool {
            false
        }

        fn c_dotstar(&mut self) -> result::Result<Patch, Error> {
            Err(Error::new("Should not call c_dotstar"))
        }

        fn fill_to_next(&mut self, _hole: Hole) {}

        fn push_split_hole(&mut self) -> usize {
            0
        }

        fn c_capture(&mut self, _arg: usize, _expr: &Hir) -> result::Result<Patch, Error> {
            Err(Error::new("Capture failed"))
        }

        fn fill(&mut self, _prev_hole: Hole, _entry: usize) {}

        fn push_compiled(&mut self, _inst: Inst) {}

        fn compile_finish(&mut self) -> result::Result<Program, Error> {
            Ok(Program {})
        }
    }

    let exprs = vec![Hir {}, Hir {}];

    let mut test_struct = TestStruct {
        compiled: Compiled {
            is_anchored_start: false,
            is_anchored_end: false,
            matches: vec![],
            start: 0,
        },
    };

    let _result = test_struct.compile_many(&exprs);
}

fn test_compile_many_no_dotstar() {
    struct TestStruct {
        compiled: Compiled,
    }

    impl TestStruct {
        fn needs_dotstar(&self) -> bool {
            false
        }

        fn c_dotstar(&mut self) -> result::Result<Patch, Error> {
            Err(Error::new("Should not call c_dotstar"))
        }

        fn fill_to_next(&mut self, _hole: Hole) {}

        fn push_split_hole(&mut self) -> usize {
            0
        }

        fn c_capture(&mut self, _arg: usize, _expr: &Hir) -> result::Result<Patch, Error> {
            Err(Error::new("Capture failed"))
        }

        fn fill(&mut self, _prev_hole: Hole, _entry: usize) {}

        fn push_compiled(&mut self, _inst: Inst) {}

        fn compile_finish(&mut self) -> result::Result<Program, Error> {
            Ok(Program {})
        }
    }

    let exprs = vec![Hir {}, Hir {}];

    let mut test_struct = TestStruct {
        compiled: Compiled {
            is_anchored_start: false,
            is_anchored_end: false,
            matches: vec![],
            start: 0,
        },
    };

    let _result = test_struct.compile_many(&exprs);
}

fn test_compile_many_panic_conditions() {
    struct TestStruct {
        compiled: Compiled,
    }

    impl TestStruct {
        fn needs_dotstar(&self) -> bool {
            false
        }

        fn c_dotstar(&mut self) -> result::Result<Patch, Error> {
            Err(Error::new("Should not call c_dotstar"))
        }

        fn fill_to_next(&mut self, _hole: Hole) {}

        fn push_split_hole(&mut self) -> usize {
            0
        }

        fn c_capture(&mut self, _arg: usize, _expr: &Hir) -> result::Result<Patch, Error> {
            Err(Error::new("Capture failed"))
        }

        fn fill(&mut self, _prev_hole: Hole, _entry: usize) {}

        fn push_compiled(&mut self, _inst: Inst) {}

        fn compile_finish(&mut self) -> result::Result<Program, Error> {
            Ok(Program {})
        }
    }

    let exprs = vec![Hir {}, Hir {}, Hir {}];

    let mut test_struct = TestStruct {
        compiled: Compiled {
            is_anchored_start: false,
            is_anchored_end: false,
            matches: vec![],
            start: 0,
        },
    };

    let panic_result = std::panic::catch_unwind(|| {
        let _result = test_struct.compile_many(&exprs[0..1]);
    });

    assert!(panic_result.is_err());
}

