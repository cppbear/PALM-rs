// Answer 0

#[test]
fn test_clone_from_spec_normal_case() {
    let mut source_table = RawTable {
        table: RawTableInner {
            bucket_mask: 16,
            ctrl: NonNull::new(unsafe { std::mem::transmute(0_usize) }).unwrap(),
            growth_left: 10,
            items: 5,
        },
        alloc: Global,
        marker: PhantomData::<u32>,
    };
    let mut target_table = RawTable {
        table: RawTableInner {
            bucket_mask: 16,
            ctrl: NonNull::new(unsafe { std::mem::transmute(0_usize) }).unwrap(),
            growth_left: 5,
            items: 2,
        },
        alloc: Global,
        marker: PhantomData::<u32>,
    };

    unsafe {
        target_table.clone_from_spec(&source_table);
    }
}

#[test]
fn test_clone_from_spec_empty_source() {
    let mut source_table = RawTable {
        table: RawTableInner {
            bucket_mask: 8,
            ctrl: NonNull::new(unsafe { std::mem::transmute(0_usize) }).unwrap(),
            growth_left: 0,
            items: 0,
        },
        alloc: Global,
        marker: PhantomData::<u32>,
    };
    let mut target_table = RawTable {
        table: RawTableInner {
            bucket_mask: 8,
            ctrl: NonNull::new(unsafe { std::mem::transmute(0_usize) }).unwrap(),
            growth_left: 5,
            items: 3,
        },
        alloc: Global,
        marker: PhantomData::<u32>,
    };

    unsafe {
        target_table.clone_from_spec(&source_table);
    }
}

#[test]
fn test_clone_from_spec_full_table() {
    let mut source_table = RawTable {
        table: RawTableInner {
            bucket_mask: 32,
            ctrl: NonNull::new(unsafe { std::mem::transmute(0_usize) }).unwrap(),
            growth_left: 0,
            items: 16,
        },
        alloc: Global,
        marker: PhantomData::<u32>,
    };
    let mut target_table = RawTable {
        table: RawTableInner {
            bucket_mask: 32,
            ctrl: NonNull::new(unsafe { std::mem::transmute(0_usize) }).unwrap(),
            growth_left: 10,
            items: 5,
        },
        alloc: Global,
        marker: PhantomData::<u32>,
    };

    unsafe {
        target_table.clone_from_spec(&source_table);
    }
}

#[test]
fn test_clone_from_spec_large_values() {
    let mut source_table = RawTable {
        table: RawTableInner {
            bucket_mask: 1 << 30,
            ctrl: NonNull::new(unsafe { std::mem::transmute(0_usize) }).unwrap(),
            growth_left: 1 << 30,
            items: 1 << 30,
        },
        alloc: Global,
        marker: PhantomData::<u32>,
    };
    let mut target_table = RawTable {
        table: RawTableInner {
            bucket_mask: 1 << 30,
            ctrl: NonNull::new(unsafe { std::mem::transmute(0_usize) }).unwrap(),
            growth_left: 1 << 30,
            items: 0,
        },
        alloc: Global,
        marker: PhantomData::<u32>,
    };

    unsafe {
        target_table.clone_from_spec(&source_table);
    }
}

#[test]
#[should_panic]
fn test_clone_from_spec_panic_on_large_source() {
    let mut source_table = RawTable {
        table: RawTableInner {
            bucket_mask: 1 << 30,
            ctrl: NonNull::new(unsafe { std::mem::transmute(0_usize) }).unwrap(),
            growth_left: 2 << 30, // exceeding allowed limits
            items: 1 << 30,
        },
        alloc: Global,
        marker: PhantomData::<u32>,
    };
    let mut target_table = RawTable {
        table: RawTableInner {
            bucket_mask: 16,
            ctrl: NonNull::new(unsafe { std::mem::transmute(0_usize) }).unwrap(),
            growth_left: 5,
            items: 5,
        },
        alloc: Global,
        marker: PhantomData::<u32>,
    };

    unsafe {
        target_table.clone_from_spec(&source_table);
    }
}

