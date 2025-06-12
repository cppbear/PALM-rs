// Answer 0

#[test]
fn test_byte_class_none() {
    struct DummyProgram {
        byte_classes: Vec<u8>,
    }

    struct DummyFsm<'a> {
        prog: &'a DummyProgram,
    }

    impl<'a> DummyFsm<'a> {
        fn num_byte_classes(&self) -> usize {
            self.prog.byte_classes.len()
        }

        fn byte_class(&self, b: Byte) -> usize {
            match b.as_byte() {
                None => self.num_byte_classes() - 1,
                Some(b) => self.u8_class(b),
            }
        }

        fn u8_class(&self, b: u8) -> usize {
            self.prog.byte_classes[b as usize] as usize
        }
    }

    let program = DummyProgram {
        byte_classes: vec![0, 1, 2, 3],
    };

    let fsm = DummyFsm { prog: &program };
    let byte_input = Byte::eof();
    
    let result = fsm.byte_class(byte_input);
    
    assert_eq!(result, fsm.num_byte_classes() - 1);
}

