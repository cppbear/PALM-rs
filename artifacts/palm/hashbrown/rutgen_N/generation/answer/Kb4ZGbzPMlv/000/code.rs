// Answer 0

#[test]
fn test_data_end_valid() {
    use std::ptr::NonNull;

    struct RawTable {
        table: Table,
    }

    struct Table {
        ctrl: *mut i32,
    }

    let mut data = vec![1, 2, 3];
    let ctrl_ptr = data.as_mut_ptr() as *mut i32;
    
    let raw_table = RawTable {
        table: Table {
            ctrl: ctrl_ptr,
        },
    };

    let data_end_ptr: NonNull<i32> = unsafe { NonNull::new_unchecked(raw_table.data_end().as_ptr()) };
    
    assert_eq!(data_end_ptr.as_ptr(), unsafe { ctrl_ptr.add(data.len()) });
}

#[test]
#[should_panic]
fn test_data_end_invalid_usage() {
    use std::ptr::NonNull;

    struct RawTable {
        table: Table,
    }

    struct Table {
        ctrl: *mut i32,
    }

    let data = vec![1, 2, 3];
    let ctrl_ptr = data.as_ptr() as *mut i32;
    
    let raw_table = RawTable {
        table: Table {
            ctrl: ctrl_ptr,
        },
    };

    let _ = raw_table.data_end(); // Pointer outlives the RawTable
}

