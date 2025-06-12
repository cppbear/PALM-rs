// Answer 0

#[test]
fn test_resize_capacity_equals() {
    struct SparseSet {
        capacity: usize,
    }

    impl SparseSet {
        fn new(capacity: usize) -> Self {
            SparseSet { capacity }
        }

        fn capacity(&self) -> usize {
            self.capacity
        }
    }

    struct MyStruct {
        set: SparseSet,
        slots_per_thread: usize,
        caps: Vec<Option<()>>,
    }

    impl MyStruct {
        fn new(initial_capacity: usize) -> Self {
            MyStruct {
                set: SparseSet::new(initial_capacity),
                slots_per_thread: 0,
                caps: Vec::new(),
            }
        }

        fn resize(&mut self, num_insts: usize, ncaps: usize) {
            if num_insts == self.set.capacity() {
                return;
            }
            self.slots_per_thread = ncaps * 2;
            self.set = SparseSet::new(num_insts);
            self.caps = vec![None; self.slots_per_thread * num_insts];
        }
    }

    let mut my_struct = MyStruct::new(5);
    my_struct.resize(5, 2);
    assert_eq!(my_struct.slots_per_thread, 0);
    assert_eq!(my_struct.caps.len(), 0);
}

#[test]
fn test_resize_increases_capacity() {
    struct SparseSet {
        capacity: usize,
    }

    impl SparseSet {
        fn new(capacity: usize) -> Self {
            SparseSet { capacity }
        }

        fn capacity(&self) -> usize {
            self.capacity
        }
    }

    struct MyStruct {
        set: SparseSet,
        slots_per_thread: usize,
        caps: Vec<Option<()>>,
    }

    impl MyStruct {
        fn new(initial_capacity: usize) -> Self {
            MyStruct {
                set: SparseSet::new(initial_capacity),
                slots_per_thread: 0,
                caps: Vec::new(),
            }
        }

        fn resize(&mut self, num_insts: usize, ncaps: usize) {
            if num_insts == self.set.capacity() {
                return;
            }
            self.slots_per_thread = ncaps * 2;
            self.set = SparseSet::new(num_insts);
            self.caps = vec![None; self.slots_per_thread * num_insts];
        }
    }

    let mut my_struct = MyStruct::new(5);
    my_struct.resize(3, 2);
    assert_eq!(my_struct.slots_per_thread, 4);
    assert_eq!(my_struct.caps.len(), 12);
}

#[test]
fn test_resize_with_different_values() {
    struct SparseSet {
        capacity: usize,
    }

    impl SparseSet {
        fn new(capacity: usize) -> Self {
            SparseSet { capacity }
        }

        fn capacity(&self) -> usize {
            self.capacity
        }
    }

    struct MyStruct {
        set: SparseSet,
        slots_per_thread: usize,
        caps: Vec<Option<()>>,
    }

    impl MyStruct {
        fn new(initial_capacity: usize) -> Self {
            MyStruct {
                set: SparseSet::new(initial_capacity),
                slots_per_thread: 0,
                caps: Vec::new(),
            }
        }

        fn resize(&mut self, num_insts: usize, ncaps: usize) {
            if num_insts == self.set.capacity() {
                return;
            }
            self.slots_per_thread = ncaps * 2;
            self.set = SparseSet::new(num_insts);
            self.caps = vec![None; self.slots_per_thread * num_insts];
        }
    }

    let mut my_struct = MyStruct::new(5);
    my_struct.resize(7, 3);
    assert_eq!(my_struct.slots_per_thread, 6);
    assert_eq!(my_struct.caps.len(), 42);
}

