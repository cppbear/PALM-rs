// Answer 0

#[test]
fn test_reinsert_entry_in_order_empty_bucket() {
    let mut header_map = HeaderMap::with_capacity(10);
    header_map.indices = Box::from([Pos::none(); 10]);
    header_map.mask = 9;
    
    let pos = Pos::new(0, HashValue(1));
    header_map.reinsert_entry_in_order(pos);
}

#[test]
fn test_reinsert_entry_in_order_multiple_empty_buckets() {
    let mut header_map = HeaderMap::with_capacity(20);
    header_map.indices = Box::from([Pos::none(); 20]);
    header_map.mask = 19;

    let pos1 = Pos::new(0, HashValue(3));
    let pos2 = Pos::new(1, HashValue(4));
    header_map.reinsert_entry_in_order(pos1);
    header_map.reinsert_entry_in_order(pos2);
}

#[test]
fn test_reinsert_entry_in_order_last_empty_bucket() {
    let mut header_map = HeaderMap::with_capacity(5);
    header_map.indices = Box::from([Pos::none(); 5]);
    header_map.mask = 4;

    let pos = Pos::new(4, HashValue(2));
    header_map.reinsert_entry_in_order(pos);
}

#[test]
fn test_reinsert_entry_in_order_no_empty_buckets() {
    let mut header_map = HeaderMap::with_capacity(3);
    header_map.indices = Box::from([
        Pos::new(0, HashValue(5)),
        Pos::new(1, HashValue(6)),
        Pos::new(2, HashValue(7)),
    ]);
    header_map.mask = 2;

    let pos = Pos::new(3, HashValue(5));
    header_map.reinsert_entry_in_order(pos);
}

#[test]
fn test_reinsert_entry_in_order_full_indices_and_empty_pos() {
    let mut header_map = HeaderMap::with_capacity(3);
    header_map.indices = Box::from([Pos::none(); 3]);
    header_map.indices[0] = Pos::new(0, HashValue(5));
    header_map.indices[1] = Pos::new(1, HashValue(5));
    header_map.indices[2] = Pos::new(2, HashValue(5));
    header_map.mask = 2;

    let pos_empty = Pos::new(3, HashValue(1));
    header_map.reinsert_entry_in_order(pos_empty);
}

