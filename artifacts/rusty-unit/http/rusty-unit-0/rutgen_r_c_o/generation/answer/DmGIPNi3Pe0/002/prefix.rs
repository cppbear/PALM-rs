// Answer 0

#[test]
fn test_do_insert_phase_two_no_displacement() {
    let mut indices = [Pos::none(); 5];
    let probe = 0;
    let old_pos = Pos::new(1, HashValue(123));

    let result = do_insert_phase_two(&mut indices, probe, old_pos);
    // The expected return value is 0 since the position at probe is none and will be filled.
}

#[test]
fn test_do_insert_phase_two_with_displacement() {
    let mut indices = [
        Pos::new(2, HashValue(456)),
        Pos::new(3, HashValue(789)),
        Pos::none(),
        Pos::new(4, HashValue(101)),
    ];
    let probe = 1;
    let old_pos = Pos::new(5, HashValue(102));

    let result = do_insert_phase_two(&mut indices, probe, old_pos);
    // The expected return value is 2 because two positions will be displaced before finding a none.
}

#[test]
fn test_do_insert_phase_two_full_displacement() {
    let mut indices = [
        Pos::new(0, HashValue(0)),
        Pos::new(1, HashValue(1)),
        Pos::new(2, HashValue(2)),
    ];
    let probe = 0;
    let old_pos = Pos::new(3, HashValue(3));

    let result = do_insert_phase_two(&mut indices, probe, old_pos);
    // The expected return value is 3 because each position will be displaced.
}

#[test]
fn test_do_insert_phase_two_edge_case_empty() {
    let mut indices: [Pos; 1] = [Pos::none()];
    let probe = 0;
    let old_pos = Pos::new(1, HashValue(111));

    let result = do_insert_phase_two(&mut indices, probe, old_pos);
    // The expected return value is 0 since the sole position is none.
}

#[test]
fn test_do_insert_phase_two_with_multi_displacement() {
    let mut indices = [
        Pos::new(5, HashValue(500)),
        Pos::none(),
        Pos::new(7, HashValue(700)),
    ];
    let probe = 0;
    let old_pos = Pos::new(8, HashValue(800));

    let result = do_insert_phase_two(&mut indices, probe, old_pos);
    // The expected return value is 2 as there will be 2 displacements before finding a none.
}

