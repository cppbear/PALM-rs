// Answer 0

#[test]
fn test_do_insert_phase_two_empty() {
    let mut indices: [Pos; 0] = [];
    let probe: usize = 0;
    let old_pos = Pos::none();
    let displaced_count = do_insert_phase_two(&mut indices, probe, old_pos);
    assert_eq!(displaced_count, 0);
}

#[test]
fn test_do_insert_phase_two_single_insert() {
    let mut indices: [Pos; 1] = [Pos::none()];
    let probe: usize = 0;
    let old_pos = Pos::new(0, HashValue(123));
    let displaced_count = do_insert_phase_two(&mut indices, probe, old_pos);
    assert_eq!(displaced_count, 0);
    assert_eq!(indices[0], old_pos);
}

#[test]
fn test_do_insert_phase_two_single_displacement() {
    let old_pos = Pos::new(0, HashValue(123));
    let mut indices: [Pos; 1] = [old_pos];
    let probe: usize = 0;
    let new_pos = Pos::new(1, HashValue(456));
    
    let displaced_count = do_insert_phase_two(&mut indices, probe, new_pos);
    assert_eq!(displaced_count, 1);
    assert_eq!(indices[0], new_pos);
    assert_eq!(indices[0].is_none(), false);
}

