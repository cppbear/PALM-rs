// Answer 0

#[test]
fn test_do_insert_phase_two_no_elements_displaced() {
    let mut indices: [Pos; 1] = [Pos::none()];
    let probe: usize = 0;
    let old_pos = Pos::new(1, HashValue(12345));
    
    let displaced = do_insert_phase_two(&mut indices, probe, old_pos);
    
    assert_eq!(displaced, 0);
    assert_eq!(indices[0], old_pos);
}

#[test]
fn test_do_insert_phase_two_one_element_displaced() {
    let mut indices: [Pos; 2] = [Pos::none(), Pos::new(0, HashValue(0))];
    let probe: usize = 0;
    let old_pos = Pos::new(1, HashValue(12345));
    
    let displaced = do_insert_phase_two(&mut indices, probe, old_pos);
    
    assert_eq!(displaced, 1);
    assert_eq!(indices[0], Pos::new(0, HashValue(0)));
    assert_eq!(indices[1], old_pos);
}

#[test]
fn test_do_insert_phase_two_multiple_elements_displaced() {
    let mut indices: [Pos; 3] = [
        Pos::new(0, HashValue(0)),
        Pos::new(1, HashValue(1)),
        Pos::none()
    ];
    let probe: usize = 0;
    let old_pos = Pos::new(2, HashValue(12345));
    
    let displaced = do_insert_phase_two(&mut indices, probe, old_pos);
    
    assert_eq!(displaced, 2);
    assert_eq!(indices[0], Pos::new(0, HashValue(0)));
    assert_eq!(indices[1], Pos::new(1, HashValue(1)));
    assert_eq!(indices[2], old_pos);
}

