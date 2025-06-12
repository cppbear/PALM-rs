// Answer 0

#[test]
fn test_calculate_layout_for_zero_size_one_bucket() {
    let layout = TableLayout::new::<u8>();
    let _ = layout.calculate_layout_for(1);
}

#[test]
fn test_calculate_layout_for_zero_size_two_buckets() {
    let layout = TableLayout::new::<u8>();
    let _ = layout.calculate_layout_for(2);
}

#[test]
fn test_calculate_layout_for_non_zero_size_two_buckets() {
    let layout = TableLayout::new::<u32>();
    let _ = layout.calculate_layout_for(2);
}

#[test]
fn test_calculate_layout_for_non_zero_size_four_buckets() {
    let layout = TableLayout::new::<u32>();
    let _ = layout.calculate_layout_for(4);
}

#[test]
fn test_calculate_layout_for_large_size_two_buckets() {
    const MAX_SIZE: usize = isize::MAX as usize / 2;
    let layout = TableLayout { size: MAX_SIZE, ctrl_align: 1 };
    let _ = layout.calculate_layout_for(2);
}

#[test]
fn test_calculate_layout_for_max_buckets() {
    let layout = TableLayout::new::<u8>();
    let _ = layout.calculate_layout_for(2usize.pow(29));
}

#[test]
#[should_panic]
fn test_calculate_layout_for_exceeding_isize() {
    const MAX_SIZE: usize = isize::MAX as usize - (Group::WIDTH + 1);
    let layout = TableLayout { size: MAX_SIZE, ctrl_align: Group::WIDTH };
    let _ = layout.calculate_layout_for(2);
}

