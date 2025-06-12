// Answer 0

#[test]
fn test_do_insert_phase_two_no_displacement() {
    let mut indices = [Pos::none(); 10];
    let probe = 0;
    let old_pos = Pos::new(5, HashValue(12345));
    let displaced = do_insert_phase_two(&mut indices, probe, old_pos);
    assert_eq!(displaced, 0);
    assert!(indices[probe].is_some());
    assert_eq!(indices[probe].index, 5);
}

#[test]
fn test_do_insert_phase_two_single_displacement() {
    let mut indices = [
        Pos::none(),
        Pos::new(1, HashValue(11111)),
        Pos::none(),
    ];
    let probe = 1;
    let old_pos = Pos::new(5, HashValue(12345));
    let displaced = do_insert_phase_two(&mut indices, probe, old_pos);
    assert_eq!(displaced, 1);
    assert!(indices[probe].is_some());
    assert_eq!(indices[probe].index, 1);
    assert!(indices[probe + 1].is_some());
    assert_eq!(indices[probe + 1].index, 5);
}

#[test]
fn test_do_insert_phase_two_multiple_displacements() {
    let mut indices = [
        Pos::new(1, HashValue(11111)),
        Pos::new(2, HashValue(22222)),
        Pos::none(),
        Pos::new(3, HashValue(33333)),
    ];
    let probe = 0;
    let old_pos = Pos::new(5, HashValue(12345));
    let displaced = do_insert_phase_two(&mut indices, probe, old_pos);
    assert_eq!(displaced, 2);
    assert!(indices[probe].is_some());
    assert_eq!(indices[probe].index, 1);
    assert!(indices[1].is_some());
    assert_eq!(indices[1].index, 2);
    assert!(indices[2].is_some());
    assert_eq!(indices[2].index, 5);
}

