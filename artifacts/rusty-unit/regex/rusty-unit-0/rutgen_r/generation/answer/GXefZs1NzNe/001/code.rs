// Answer 0

#[test]
fn test_push_split_hole() {
    struct TestStruct {
        insts: Vec<MaybeInst>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                insts: Vec::new(),
            }
        }

        fn push_split_hole(&mut self) -> Hole {
            let hole = self.insts.len();
            self.insts.push(MaybeInst::Split);
            Hole::One(hole)
        }
    }

    let mut test_instance = TestStruct::new();

    // Verify the initial state returns Hole::One with the correct index (0).
    let result = test_instance.push_split_hole();
    assert_eq!(result, Hole::One(0));
    assert_eq!(test_instance.insts.len(), 1);

    // Call the function again and verify it returns Hole::One with the new index (1).
    let result = test_instance.push_split_hole();
    assert_eq!(result, Hole::One(1));
    assert_eq!(test_instance.insts.len(), 2);

    // Call it one more time and verify the return value and state.
    let result = test_instance.push_split_hole();
    assert_eq!(result, Hole::One(2));
    assert_eq!(test_instance.insts.len(), 3);
}

