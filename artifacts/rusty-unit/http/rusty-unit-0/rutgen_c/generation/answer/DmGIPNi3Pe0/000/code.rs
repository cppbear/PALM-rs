// Answer 0

#[test]
fn test_do_insert_phase_two_new_position_none() {
    let mut indices = [Pos::none(); 10];  // Create an array of Positions, all set to none
    let probe = 0;
    let old_pos = Pos::new(1, HashValue(10));  // Create a new position

    let displaced_count = do_insert_phase_two(&mut indices, probe, old_pos);
    
    assert_eq!(displaced_count, 0);
    assert!(indices[0].is_some());
    assert_eq!(indices[0].index, 1);
    assert_eq!(indices[0].hash.0, 10);  // Assuming HashValue(10) can be accessed via .0
}

#[test]
fn test_do_insert_phase_two_displace_one() {
    let mut indices = [Pos::new(0, HashValue(5)); 10];  // Create an array with one occupied position
    let probe = 0;
    let old_pos = Pos::new(1, HashValue(10));  // New position to insert

    let displaced_count = do_insert_phase_two(&mut indices, probe, old_pos);
    
    assert_eq!(displaced_count, 1);
    assert!(indices[0].is_some());
    assert_eq!(indices[0].index, 1);
    assert_eq!(indices[0].hash.0, 10);
    assert_eq!(indices[1].index, 0);  // The previously occupied position is now displaced
    assert_eq!(indices[1].hash.0, 5);
}

#[test]
fn test_do_insert_phase_two_full() {
    let mut indices = [
        Pos::new(0, HashValue(5)),
        Pos::new(1, HashValue(6)),
        Pos::new(2, HashValue(7)),
        Pos::new(3, HashValue(8)),
        Pos::new(4, HashValue(9)),
        Pos::new(5, HashValue(10)),
        Pos::new(6, HashValue(11)),
        Pos::new(7, HashValue(12)),
        Pos::new(8, HashValue(13)),
        Pos::new(9, HashValue(14))
    ];  // Create an array full of positions
    let probe = 0;
    let old_pos = Pos::new(10, HashValue(15));  // New position to insert

    let displaced_count = do_insert_phase_two(&mut indices, probe, old_pos);
    
    assert_eq!(displaced_count, 10);  // All 10 should get displaced
    assert_eq!(indices[0].index, 10);
    assert_eq!(indices[0].hash.0, 15);
}

#[test]
fn test_do_insert_phase_two_no_displacement() {
    let mut indices = [Pos::none(); 10];  // Create an array with all elements 'none'
    let probe = 0;
    let old_pos = Pos::new(0, HashValue(5));  // New position to insert

    let displaced_count = do_insert_phase_two(&mut indices, probe, old_pos);
    
    assert_eq!(displaced_count, 0);
    assert!(indices[0].is_some());
    assert_eq!(indices[0].index, 0);
    assert_eq!(indices[0].hash.0, 5);
}

