// Answer 0

#[test]
fn test_do_insert_phase_two_none_case() {
    use std::mem;

    struct Pos(Option<i32>);

    let mut indices = [
        Pos(None),
        Pos(Some(1)),
        Pos(Some(2)),
    ];
    let probe = 0;
    let old_pos = Pos(Some(99));

    let displaced = do_insert_phase_two(&mut indices, probe, old_pos);

    assert_eq!(displaced, 0);
    assert_eq!(indices[0], Pos(Some(99)));
}

#[test]
fn test_do_insert_phase_two_displace_one() {
    use std::mem;

    struct Pos(Option<i32>);

    let mut indices = [
        Pos(Some(1)),
        Pos(None),
        Pos(Some(3)),
    ];
    let probe = 0;
    let old_pos = Pos(Some(99));

    let displaced = do_insert_phase_two(&mut indices, probe, old_pos);

    assert_eq!(displaced, 1);
    assert_eq!(indices[0], Pos(Some(99)));
    assert_eq!(indices[1], Pos(Some(1)));
}

#[test]
fn test_do_insert_phase_two_displace_multiple() {
    use std::mem;

    struct Pos(Option<i32>);

    let mut indices = [
        Pos(Some(1)),
        Pos(Some(2)),
        Pos(Some(3)),
    ];
    let probe = 0;
    let old_pos = Pos(Some(99));

    let displaced = do_insert_phase_two(&mut indices, probe, old_pos);

    assert_eq!(displaced, 3);
    assert_eq!(indices[0], Pos(Some(99)));
    assert_eq!(indices[1], Pos(Some(1)));
    assert_eq!(indices[2], Pos(Some(2)));
}

#[test]
fn test_do_insert_phase_two_empty_indices_and_valid_probe() {
    use std::mem;

    struct Pos(Option<i32>);

    let mut indices: [Pos; 1] = [Pos(None)];
    let probe = 0;
    let old_pos = Pos(Some(99));

    let displaced = do_insert_phase_two(&mut indices, probe, old_pos);

    assert_eq!(displaced, 0);
    assert_eq!(indices[0], Pos(Some(99)));
}

