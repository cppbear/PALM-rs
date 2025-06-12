// Answer 0

#[test]
fn test_next_unchecked_valid() {
    struct Transitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl Transitions {
        fn new(num_byte_classes: usize) -> Transitions {
            let table_size = 10;
            Transitions {
                table: (0..table_size as StatePtr).collect(),
                num_byte_classes,
            }
        }

        fn num_states(&self) -> usize {
            self.table.len()
        }

        unsafe fn next_unchecked(&self, si: StatePtr, cls: usize) -> StatePtr {
            debug_assert!((si as usize) < self.table.len());
            debug_assert!(cls < self.num_byte_classes);
            *self.table.get_unchecked(si as usize + cls)
        }
    }

    let transitions = Transitions::new(5);
    let si: StatePtr = 2;
    let cls: usize = 1;
    let result: StatePtr;

    unsafe {
        result = transitions.next_unchecked(si, cls);
    }

    assert_eq!(result, 3); // 2 + 1 = 3
}

#[test]
#[should_panic]
fn test_next_unchecked_si_out_of_bounds() {
    struct Transitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl Transitions {
        fn new(num_byte_classes: usize) -> Transitions {
            let table_size = 10;
            Transitions {
                table: (0..table_size as StatePtr).collect(),
                num_byte_classes,
            }
        }

        fn num_states(&self) -> usize {
            self.table.len()
        }

        unsafe fn next_unchecked(&self, si: StatePtr, cls: usize) -> StatePtr {
            debug_assert!((si as usize) < self.table.len());
            debug_assert!(cls < self.num_byte_classes);
            *self.table.get_unchecked(si as usize + cls)
        }
    }

    let transitions = Transitions::new(5);
    let si: StatePtr = 10; // Out of bounds
    let cls: usize = 0;

    unsafe {
        transitions.next_unchecked(si, cls);
    }
}

#[test]
#[should_panic]
fn test_next_unchecked_cls_out_of_bounds() {
    struct Transitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl Transitions {
        fn new(num_byte_classes: usize) -> Transitions {
            let table_size = 10;
            Transitions {
                table: (0..table_size as StatePtr).collect(),
                num_byte_classes,
            }
        }

        fn num_states(&self) -> usize {
            self.table.len()
        }

        unsafe fn next_unchecked(&self, si: StatePtr, cls: usize) -> StatePtr {
            debug_assert!((si as usize) < self.table.len());
            debug_assert!(cls < self.num_byte_classes);
            *self.table.get_unchecked(si as usize + cls)
        }
    }

    let transitions = Transitions::new(5);
    let si: StatePtr = 1;
    let cls: usize = 5; // Out of bounds

    unsafe {
        transitions.next_unchecked(si, cls);
    }
}

