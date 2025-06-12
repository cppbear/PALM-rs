// Answer 0

#[test]
fn test_do_insert_phase_two_with_empty_indices() {
    let mut indices = vec![None; 5];
    let probe = 0;
    let old_pos = Some(1); // Example position to insert

    let displaced = do_insert_phase_two(&mut indices, probe, old_pos);
    
    assert_eq!(displaced, 0);
    assert_eq!(indices, vec![Some(1), None, None, None, None]);
}

#[test]
fn test_do_insert_phase_two_with_one_displaced() {
    let mut indices = vec![Some(2), None, None, None, None];
    let probe = 0;
    let old_pos = Some(1);

    let displaced = do_insert_phase_two(&mut indices, probe, old_pos);
    
    assert_eq!(displaced, 1);
    assert_eq!(indices, vec![Some(1), Some(2), None, None, None]);
}

#[test]
fn test_do_insert_phase_two_with_multiple_displaced() {
    let mut indices = vec![Some(2), Some(3), None, None, None];
    let probe = 0;
    let old_pos = Some(1);

    let displaced = do_insert_phase_two(&mut indices, probe, old_pos);
    
    assert_eq!(displaced, 2);
    assert_eq!(indices, vec![Some(1), Some(2), Some(3), None, None]);
}

#[test]
fn test_do_insert_phase_two_when_filling_up() {
    let mut indices = vec![Some(1), Some(2), Some(3), None, None];
    let probe = 2;
    let old_pos = Some(4);

    let displaced = do_insert_phase_two(&mut indices, probe, old_pos);
    
    assert_eq!(displaced, 1);
    assert_eq!(indices, vec![Some(1), Some(2), Some(4), Some(3), None]);
} 

#[test]
#[should_panic]
fn test_do_insert_phase_two_panic_out_of_bounds() {
    let mut indices = vec![Some(1), Some(2), Some(3)];
    let probe = 3; // Out of bounds
    let old_pos = Some(4);

    do_insert_phase_two(&mut indices, probe, old_pos);
}

