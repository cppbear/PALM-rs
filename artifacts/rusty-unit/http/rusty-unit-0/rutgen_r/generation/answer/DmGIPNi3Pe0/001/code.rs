// Answer 0

#[test]
fn test_do_insert_phase_two_no_displacement() {
    let mut indices = vec![Some(Pos(1)), None, None];
    let probe = 1;
    let old_pos = Pos(2);

    let num_displaced = do_insert_phase_two(&mut indices, probe, old_pos);

    assert_eq!(num_displaced, 0);
    assert_eq!(indices[probe], Some(Pos(2)));
}

#[test]
fn test_do_insert_phase_two_one_displacement() {
    let mut indices = vec![Some(Pos(1)), Some(Pos(2)), None];
    let probe = 1;
    let old_pos = Pos(3);

    let num_displaced = do_insert_phase_two(&mut indices, probe, old_pos);

    assert_eq!(num_displaced, 1);
    assert_eq!(indices[probe], Some(Pos(3)));
    assert_eq!(indices[0], Some(Pos(1)));
}

#[test]
fn test_do_insert_phase_two_multiple_displacements() {
    let mut indices = vec![Some(Pos(1)), Some(Pos(2)), Some(Pos(3)), None];
    let probe = 0; 
    let old_pos = Pos(4);

    let num_displaced = do_insert_phase_two(&mut indices, probe, old_pos);

    assert_eq!(num_displaced, 3);
    assert_eq!(indices[0], Some(Pos(4)));
    assert_eq!(indices[1], Some(Pos(1)));
    assert_eq!(indices[2], Some(Pos(2)));
}

#[test]
#[should_panic]
fn test_do_insert_phase_two_invalid_probe() {
    let mut indices = vec![Some(Pos(1)), Some(Pos(2))];
    let probe = 2; // Out of bounds
    let old_pos = Pos(3);

    let _ = do_insert_phase_two(&mut indices, probe, old_pos);
}

