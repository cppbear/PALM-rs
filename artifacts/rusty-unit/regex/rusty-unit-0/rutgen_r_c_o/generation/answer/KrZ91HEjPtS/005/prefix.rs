// Answer 0

#[test]
fn test_fill_inst_hole_save() {
    let slot = 0;
    let goto: InstPtr = 1;
    let inst_hole = InstHole::Save { slot };
    inst_hole.fill(goto);
}

#[test]
fn test_fill_inst_hole_save_mid_range() {
    let slot = 500;
    let goto: InstPtr = 250;
    let inst_hole = InstHole::Save { slot };
    inst_hole.fill(goto);
}

#[test]
fn test_fill_inst_hole_save_upper_bound() {
    let slot = 1000;
    let goto: InstPtr = 1000;
    let inst_hole = InstHole::Save { slot };
    inst_hole.fill(goto);
}

#[test]
fn test_fill_inst_hole_save_empty_case() {
    let slot = 0;
    let goto: InstPtr = 0;
    let inst_hole = InstHole::Save { slot };
    inst_hole.fill(goto);
}

#[test]
#[should_panic]
fn test_fill_inst_hole_save_negative_slot() {
    let slot = -1; // This will panic as it's out of bounds
    let goto: InstPtr = 1;
    let inst_hole = InstHole::Save { slot };
    inst_hole.fill(goto);
}

#[test]
#[should_panic]
fn test_fill_inst_hole_save_over_limit_slot() {
    let slot = 1001; // This will panic as it's out of bounds
    let goto: InstPtr = 1;
    let inst_hole = InstHole::Save { slot };
    inst_hole.fill(goto);
}

