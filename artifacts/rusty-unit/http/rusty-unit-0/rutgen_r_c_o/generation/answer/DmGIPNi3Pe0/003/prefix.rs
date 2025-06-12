// Answer 0

#[test]
fn test_do_insert_phase_two_empty() {
    let mut indices: Vec<Pos> = vec![];
    let probe: usize = 0;
    let old_pos = Pos::none();
    let _ = do_insert_phase_two(&mut indices, probe, old_pos);
}

#[test]
fn test_do_insert_phase_two_single_element() {
    let mut indices: Vec<Pos> = vec![Pos::none()];
    let probe: usize = 0;
    let old_pos = Pos::new(1, HashValue(123));
    let _ = do_insert_phase_two(&mut indices, probe, old_pos);
}

#[test]
fn test_do_insert_phase_two_multiple_elements() {
    let mut indices: Vec<Pos> = vec![Pos::none(), Pos::none(), Pos::none()];
    let probe: usize = 0;
    let old_pos = Pos::new(2, HashValue(456));
    let _ = do_insert_phase_two(&mut indices, probe, old_pos);
} 

#[test]
fn test_do_insert_phase_two_displaced_elements() {
    let mut indices: Vec<Pos> = vec![Pos::new(0, HashValue(1)), Pos::new(1, HashValue(2)), Pos::new(2, HashValue(3))];
    let probe: usize = 0;
    let old_pos = Pos::new(3, HashValue(4));
    let _ = do_insert_phase_two(&mut indices, probe, old_pos);
} 

#[test]
fn test_do_insert_phase_two_full_capacity() {
    let mut indices: Vec<Pos> = vec![Pos::new(0, HashValue(1)); 32767];
    let probe: usize = 0;
    let old_pos = Pos::new(32766, HashValue(999));
    let _ = do_insert_phase_two(&mut indices, probe, old_pos);
}

