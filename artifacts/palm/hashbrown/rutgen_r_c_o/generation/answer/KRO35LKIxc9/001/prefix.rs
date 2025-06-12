// Answer 0

#[test]
fn test_ctrl_valid_index_zero() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let mut table = RawTableInner::with_capacity(&alloc, table_layout, 8);
    let index = 0;
    unsafe {
        let _ptr = table.ctrl(index);
    }
}

#[test]
fn test_ctrl_valid_index_middle() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let mut table = RawTableInner::with_capacity(&alloc, table_layout, 8);
    let index = table.num_ctrl_bytes() / 2;
    unsafe {
        let _ptr = table.ctrl(index);
    }
}

#[test]
fn test_ctrl_valid_index_max() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let mut table = RawTableInner::with_capacity(&alloc, table_layout, 8);
    let index = table.num_ctrl_bytes() - 1;
    unsafe {
        let _ptr = table.ctrl(index);
    }
}

#[test]
#[should_panic]
fn test_ctrl_invalid_index_equal() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let mut table = RawTableInner::with_capacity(&alloc, table_layout, 8);
    let index = table.num_ctrl_bytes();
    unsafe {
        let _ptr = table.ctrl(index);
    }
}

#[test]
#[should_panic]
fn test_ctrl_invalid_index_out_of_bounds() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let mut table = RawTableInner::with_capacity(&alloc, table_layout, 8);
    let index = table.num_ctrl_bytes() + 1;
    unsafe {
        let _ptr = table.ctrl(index);
    }
}

