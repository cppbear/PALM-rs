// Answer 0

#[derive(Copy, Clone)]
struct LargeAlignedStruct {
    a: [u64; 10],
    b: u32,
}

#[derive(Copy, Clone)]
struct AnotherStruct {
    c: f64,
    d: [u8; 32],
}

#[test]
fn test_table_layout_large_aligned_struct() {
    let layout = TableLayout::new::<LargeAlignedStruct>();
}

#[test]
fn test_table_layout_another_struct() {
    let layout = TableLayout::new::<AnotherStruct>();
}

