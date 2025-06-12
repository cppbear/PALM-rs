// Answer 0


struct EmptyLook;

struct InstHole {
    look: EmptyLook,
}

struct Patch {
    hole: usize,
    entry: usize,
}

struct TestStruct {
    insts: Vec<InstHole>,
}

impl TestStruct {
    fn push_hole(&mut self, hole: InstHole) -> usize {
        self.insts.push(hole);
        self.insts.len() - 1
    }

    fn c_empty_look(&mut self, look: EmptyLook) -> Result<Patch, &'static str> {
        let hole = self.push_hole(InstHole { look: look });
        Ok(Patch { hole: hole, entry: self.insts.len() - 1 })
    }
}

#[test]
fn test_c_empty_look() {
    let mut test_struct = TestStruct { insts: Vec::new() };
    let look = EmptyLook;
    
    let result = test_struct.c_empty_look(look).unwrap();
    
    assert_eq!(result.hole, 0);
    assert_eq!(result.entry, 0);
}

#[test]
fn test_c_empty_look_multiple_calls() {
    let mut test_struct = TestStruct { insts: Vec::new() };
    
    let first_look = EmptyLook;
    let first_result = test_struct.c_empty_look(first_look).unwrap();
    
    assert_eq!(first_result.hole, 0);
    assert_eq!(first_result.entry, 0);

    let second_look = EmptyLook;
    let second_result = test_struct.c_empty_look(second_look).unwrap();

    assert_eq!(second_result.hole, 1);
    assert_eq!(second_result.entry, 1);
}


