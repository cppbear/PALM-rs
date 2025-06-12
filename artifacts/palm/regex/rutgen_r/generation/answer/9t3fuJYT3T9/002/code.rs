// Answer 0

#[test]
fn test_resize_increasing_capacity() {
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
            let set = SparseSet::new(initial_capacity);
            let slots_per_thread = 0;
            let caps = vec![];

            MyStruct { set, slots_per_thread, caps }
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
    my_struct.resize(10, 3);
    
    assert_eq!(my_struct.set.capacity(), 10);
    assert_eq!(my_struct.slots_per_thread, 6);
    assert_eq!(my_struct.caps.len(), 60);
}

#[test]
fn test_resize_no_change() {
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
            let set = SparseSet::new(initial_capacity);
            let slots_per_thread = 0;
            let caps = vec![];

            MyStruct { set, slots_per_thread, caps }
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
    my_struct.resize(5, 3);
    
    assert_eq!(my_struct.set.capacity(), 5);
    assert_eq!(my_struct.slots_per_thread, 0);
    assert_eq!(my_struct.caps.len(), 0);
}

#[test]
fn test_resize_with_zero_insts() {
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
            let set = SparseSet::new(initial_capacity);
            let slots_per_thread = 0;
            let caps = vec![];

            MyStruct { set, slots_per_thread, caps }
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
    my_struct.resize(0, 2);
    
    assert_eq!(my_struct.set.capacity(), 0);
    assert_eq!(my_struct.slots_per_thread, 4);
    assert_eq!(my_struct.caps.len(), 0);
}

