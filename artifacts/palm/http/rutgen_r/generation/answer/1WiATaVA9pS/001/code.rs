// Answer 0

struct Danger;

impl Danger {
    const Green: Danger = Danger;
    const Yellow: Danger = Danger;
}

impl PartialEq for Danger {
    fn eq(&self, other: &Self) -> bool {
        // Simplified equality check for these consts
        std::ptr::eq(self, other)
    }
}

fn set_yellow(danger: &mut Danger) {
    if let Danger::Green = *danger {
        *danger = Danger::Yellow;
    }
}

#[test]
fn test_set_yellow_changes_green_to_yellow() {
    let mut danger = Danger::Green;
    set_yellow(&mut danger);
    assert_eq!(danger, Danger::Yellow);
}

#[test]
fn test_set_yellow_no_change_when_already_yellow() {
    let mut danger = Danger::Yellow;
    set_yellow(&mut danger);
    assert_eq!(danger, Danger::Yellow);
}

