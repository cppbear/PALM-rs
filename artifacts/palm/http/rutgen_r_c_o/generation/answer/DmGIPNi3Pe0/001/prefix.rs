// Answer 0

#[test]
fn test_do_insert_phase_two_no_displacement() {
    let mut indices = [Pos::none(); 10];
    let probe = 0;
    let old_pos = Pos::new(5, HashValue(123));
    let num_displaced = do_insert_phase_two(&mut indices, probe, old_pos);
}

#[test]
fn test_do_insert_phase_two_single_displacement() {
    let mut indices = [Pos::none(); 10];
    indices[0] = Pos::new(1, HashValue(456));
    let probe = 0;
    let old_pos = Pos::new(5, HashValue(123));
    let num_displaced = do_insert_phase_two(&mut indices, probe, old_pos);
}

#[test]
fn test_do_insert_phase_two_multiple_displacement() {
    let mut indices = [Pos::new(1, HashValue(456)); 10]; 
    let probe = 0;
    let old_pos = Pos::new(5, HashValue(123));
    let num_displaced = do_insert_phase_two(&mut indices, probe, old_pos);
}

#[test]
fn test_do_insert_phase_two_max_capacity() {
    let mut indices = [Pos::none(); 32768];
    let probe = 0;
    let old_pos = Pos::new(1, HashValue(999));
    let num_displaced = do_insert_phase_two(&mut indices, probe, old_pos);
}

#[test]
fn test_do_insert_phase_two_mid_capacity() {
    let mut indices = [Pos::none(); 16384];
    let probe = 0;
    let old_pos = Pos::new(100, HashValue(321));
    let num_displaced = do_insert_phase_two(&mut indices, probe, old_pos);
}

