// Answer 0

#[test]
fn test_prepare_rehash_in_place_with_full_and_deleted() {
    struct Group {
        control: Vec<u8>,
    }

    impl Group {
        const WIDTH: usize = 4;

        fn load_aligned(control_bytes: &Vec<u8>, index: usize) -> Self {
            let start = index..index + Self::WIDTH;
            Self { control: control_bytes[start].to_vec() }
        }

        fn convert_special_to_empty_and_full_to_deleted(&mut self) {
            for byte in &mut self.control {
                *byte = match *byte {
                    0 => 0, // Tag::EMPTY
                    1 => 0, // Tag::DELETED
                    _ => 2, // FULL -> Tag::DELETED
                }
            }
        }

        fn store_aligned(&self, control_bytes: &mut Vec<u8>, index: usize) {
            let start = index..index + Self::WIDTH;
            control_bytes[start].copy_from_slice(&self.control);
        }
    }

    struct RawTableInner {
        buckets: usize,
        ctrl: Vec<u8>,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.buckets
        }

        fn ctrl(&mut self, index: usize) -> &mut Vec<u8> {
            &mut self.ctrl
        }

        unsafe fn prepare_rehash_in_place(&mut self) {
            for i in (0..self.buckets()).step_by(Group::WIDTH) {
                let mut group = Group::load_aligned(&self.ctrl, i);
                group.convert_special_to_empty_and_full_to_deleted();
                group.store_aligned(&mut self.ctrl, i);
            }

            if self.buckets() < Group::WIDTH {
                self.ctrl[0..self.buckets()].copy_from_slice(&self.ctrl[Group::WIDTH..Group::WIDTH + self.buckets()]);
            } else {
                self.ctrl[0..Group::WIDTH].copy_from_slice(&self.ctrl[self.buckets()..self.buckets() + Group::WIDTH]);
            }
        }
    }

    let mut raw_table = RawTableInner {
        buckets: 8,
        ctrl: vec![1, 0, 2, 1, 0, 2, 1, 1],
    };

    unsafe {
        raw_table.prepare_rehash_in_place();
    }

    assert_eq!(raw_table.ctrl, vec![0, 0, 2, 0, 0, 2, 2, 2]);
}

#[test]
#[should_panic]
fn test_prepare_rehash_in_place_with_insufficient_buckets() {
    struct Group {
        control: Vec<u8>,
    }

    impl Group {
        const WIDTH: usize = 4;

        fn load_aligned(control_bytes: &Vec<u8>, index: usize) -> Self {
            let start = index..index + Self::WIDTH;
            Self { control: control_bytes[start].to_vec() }
        }

        fn convert_special_to_empty_and_full_to_deleted(&mut self) {
            for byte in &mut self.control {
                *byte = match *byte {
                    0 => 0, // Tag::EMPTY
                    1 => 0, // Tag::DELETED
                    _ => 2, // FULL -> Tag::DELETED
                }
            }
        }

        fn store_aligned(&self, control_bytes: &mut Vec<u8>, index: usize) {
            let start = index..index + Self::WIDTH;
            control_bytes[start].copy_from_slice(&self.control);
        }
    }

    struct RawTableInner {
        buckets: usize,
        ctrl: Vec<u8>,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.buckets
        }

        fn ctrl(&mut self, index: usize) -> &mut Vec<u8> {
            &mut self.ctrl
        }

        unsafe fn prepare_rehash_in_place(&mut self) {
            for i in (0..self.buckets()).step_by(Group::WIDTH) {
                let mut group = Group::load_aligned(&self.ctrl, i);
                group.convert_special_to_empty_and_full_to_deleted();
                group.store_aligned(&mut self.ctrl, i);
            }

            if self.buckets() < Group::WIDTH {
                self.ctrl[0..self.buckets()].copy_from_slice(&self.ctrl[Group::WIDTH..Group::WIDTH + self.buckets()]);
            } else {
                self.ctrl[0..Group::WIDTH].copy_from_slice(&self.ctrl[self.buckets()..self.buckets() + Group::WIDTH]);
            }
        }
    }

    let mut raw_table = RawTableInner {
        buckets: 2,
        ctrl: vec![1, 0, 2, 1],
    };

    unsafe {
        raw_table.prepare_rehash_in_place();
    }
}

#[test]
fn test_prepare_rehash_in_place_with_zero_buckets() {
    struct Group {
        control: Vec<u8>,
    }

    impl Group {
        const WIDTH: usize = 4;

        fn load_aligned(control_bytes: &Vec<u8>, index: usize) -> Self {
            let start = index..index + Self::WIDTH;
            Self { control: control_bytes[start].to_vec() }
        }

        fn convert_special_to_empty_and_full_to_deleted(&mut self) {
            for byte in &mut self.control {
                *byte = match *byte {
                    0 => 0, // Tag::EMPTY
                    1 => 0, // Tag::DELETED
                    _ => 2, // FULL -> Tag::DELETED
                }
            }
        }

        fn store_aligned(&self, control_bytes: &mut Vec<u8>, index: usize) {
            let start = index..index + Self::WIDTH;
            control_bytes[start].copy_from_slice(&self.control);
        }
    }

    struct RawTableInner {
        buckets: usize,
        ctrl: Vec<u8>,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.buckets
        }

        fn ctrl(&mut self, index: usize) -> &mut Vec<u8> {
            &mut self.ctrl
        }

        unsafe fn prepare_rehash_in_place(&mut self) {
            for i in (0..self.buckets()).step_by(Group::WIDTH) {
                let mut group = Group::load_aligned(&self.ctrl, i);
                group.convert_special_to_empty_and_full_to_deleted();
                group.store_aligned(&mut self.ctrl, i);
            }

            if self.buckets() < Group::WIDTH {
                self.ctrl[0..self.buckets()].copy_from_slice(&self.ctrl[Group::WIDTH..Group::WIDTH + self.buckets()]);
            } else {
                self.ctrl[0..Group::WIDTH].copy_from_slice(&self.ctrl[self.buckets()..self.buckets() + Group::WIDTH]);
            }
        }
    }

    let mut raw_table = RawTableInner {
        buckets: 0,
        ctrl: vec![],
    };

    unsafe {
        raw_table.prepare_rehash_in_place();
    }
}

