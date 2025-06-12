// Answer 0

#[test]
fn test_table_layout_new_with_u8() {
    struct TestType;
    let layout = TableLayout::new::<TestType>();
    assert_eq!(layout.size, std::mem::size_of::<TestType>());
    assert_eq!(layout.ctrl_align, std::cmp::max(std::mem::align_of::<TestType>(), Group::WIDTH));
}

#[test]
fn test_table_layout_new_with_u32() {
    struct TestType;
    let layout = TableLayout::new::<TestType>();
    assert_eq!(layout.size, std::mem::size_of::<TestType>());
    assert_eq!(layout.ctrl_align, std::cmp::max(std::mem::align_of::<TestType>(), Group::WIDTH));
}

#[test]
fn test_table_layout_new_with_large_struct() {
    struct LargeStruct {
        a: [u8; 100],
        b: u64,
    }
    let layout = TableLayout::new::<LargeStruct>();
    assert_eq!(layout.size, std::mem::size_of::<LargeStruct>());
    assert_eq!(layout.ctrl_align, std::cmp::max(std::mem::align_of::<LargeStruct>(), Group::WIDTH));
}

#[test]
fn test_table_layout_new_with_empty_struct() {
    struct EmptyStruct;
    let layout = TableLayout::new::<EmptyStruct>();
    assert_eq!(layout.size, std::mem::size_of::<EmptyStruct>());
    assert_eq!(layout.ctrl_align, std::cmp::max(std::mem::align_of::<EmptyStruct>(), Group::WIDTH));
}

