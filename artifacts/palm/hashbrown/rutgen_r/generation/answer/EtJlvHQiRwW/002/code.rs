// Answer 0

#[test]
#[should_panic]
fn test_special_is_empty_when_not_special() {
    struct ControlValue(u8);

    impl ControlValue {
        const fn is_special(self) -> bool {
            false
        }

        const fn special_is_empty(self) -> bool {
            debug_assert!(self.is_special());
            self.0 & 0x01 != 0
        }
    }

    let cv = ControlValue(0x00);
    cv.special_is_empty();
}

#[test]
fn test_special_is_empty_when_special_and_non_empty() {
    struct ControlValue(u8);

    impl ControlValue {
        const fn is_special(self) -> bool {
            true
        }

        const fn special_is_empty(self) -> bool {
            debug_assert!(self.is_special());
            self.0 & 0x01 != 0
        }
    }

    let cv = ControlValue(0x01);
    assert!(cv.special_is_empty());
}

#[test]
fn test_special_is_empty_when_special_and_empty() {
    struct ControlValue(u8);

    impl ControlValue {
        const fn is_special(self) -> bool {
            true
        }

        const fn special_is_empty(self) -> bool {
            debug_assert!(self.is_special());
            self.0 & 0x01 == 0
        }
    }

    let cv = ControlValue(0x00);
    assert!(!cv.special_is_empty());
}

