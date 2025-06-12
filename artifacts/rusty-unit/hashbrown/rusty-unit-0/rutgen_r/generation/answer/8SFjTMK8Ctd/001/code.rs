// Answer 0

#[test]
fn test_erase_bucket_full_edge_case() {
    struct Group {
        data: [u8; 4],
    }

    impl Group {
        const WIDTH: usize = 4;

        fn load(ctrl: u8) -> Self {
            Group { data: [ctrl; Self::WIDTH] }
        }

        fn match_empty(&self) -> usize {
            self.data.iter().filter(|&&x| x == Tag::EMPTY as u8).count()
        }
        
        fn leading_zeros(&self) -> usize {
            self.data.iter().take_while(|&&x| x == Tag::EMPTY as u8).count()
        }

        fn trailing_zeros(&self) -> usize {
            self.data.iter().rev().take_while(|&&x| x == Tag::EMPTY as u8).count()
        }
    }

    #[derive(Clone, Copy)]
    enum Tag {
        EMPTY = 0,
        DELETED = 1,
        FULL = 2,
    }

    struct RawTableInner {
        ctrl_data: Vec<u8>,
        items: usize,
        growth_left: usize,
    }

    impl RawTableInner {
        fn new(size: usize) -> Self {
            let ctrl_data = vec![Tag::FULL as u8; size];
            Self {
                ctrl_data,
                items: size,
                growth_left: 0,
            }
        }

        fn is_bucket_full(&self, index: usize) -> bool {
            self.ctrl_data[index] == Tag::FULL as u8
        }

        fn buckets(&self) -> usize {
            self.ctrl_data.len()
        }

        fn ctrl(&self, index: usize) -> u8 {
            self.ctrl_data[index]
        }

        unsafe fn set_ctrl(&mut self, index: usize, tag: Tag) {
            self.ctrl_data[index] = tag as u8;
        }

        unsafe fn erase(&mut self, index: usize) {
            debug_assert!(self.is_bucket_full(index));
            let index_before = index.wrapping_sub(Group::WIDTH) & (self.buckets() - 1);
            let empty_before = Group::load(self.ctrl(index_before)).match_empty();
            let empty_after = Group::load(self.ctrl(index)).match_empty();

            let ctrl = if empty_before.leading_zeros() + empty_after.trailing_zeros() >= Group::WIDTH {
                Tag::DELETED
            } else {
                self.growth_left += 1;
                Tag::EMPTY
            };

            self.set_ctrl(index, ctrl);
            self.items -= 1;
        }
    }

    let size = 8; // Adjust as necessary for test
    let mut table = RawTableInner::new(size);

    unsafe {
        // Scenario meets the conditions for the test
        table.erase(0); // First call should be safe and valid
        // Ensure the control byte is marked as DELETED
        assert_eq!(table.ctrl_data[0], Tag::DELETED as u8);
        assert_eq!(table.items, size - 1);
        assert_eq!(table.growth_left, 0);
    }
}

#[test]
fn test_erase_bucket_full_with_growth_left() {
    struct Group {
        data: [u8; 4],
    }

    impl Group {
        const WIDTH: usize = 4;

        fn load(ctrl: u8) -> Self {
            Group { data: [ctrl; Self::WIDTH] }
        }

        fn match_empty(&self) -> usize {
            self.data.iter().filter(|&&x| x == Tag::EMPTY as u8).count()
        }
        
        fn leading_zeros(&self) -> usize {
            self.data.iter().take_while(|&&x| x == Tag::EMPTY as u8).count()
        }

        fn trailing_zeros(&self) -> usize {
            self.data.iter().rev().take_while(|&&x| x == Tag::EMPTY as u8).count()
        }
    }

    #[derive(Clone, Copy)]
    enum Tag {
        EMPTY = 0,
        DELETED = 1,
        FULL = 2,
    }

    struct RawTableInner {
        ctrl_data: Vec<u8>,
        items: usize,
        growth_left: usize,
    }

    impl RawTableInner {
        fn new(size: usize) -> Self {
            let ctrl_data = vec![Tag::FULL as u8; size];
            Self {
                ctrl_data,
                items: size,
                growth_left: 0,
            }
        }

        fn is_bucket_full(&self, index: usize) -> bool {
            self.ctrl_data[index] == Tag::FULL as u8
        }

        fn buckets(&self) -> usize {
            self.ctrl_data.len()
        }

        fn ctrl(&self, index: usize) -> u8 {
            self.ctrl_data[index]
        }

        unsafe fn set_ctrl(&mut self, index: usize, tag: Tag) {
            self.ctrl_data[index] = tag as u8;
        }

        unsafe fn erase(&mut self, index: usize) {
            debug_assert!(self.is_bucket_full(index));
            let index_before = index.wrapping_sub(Group::WIDTH) & (self.buckets() - 1);
            let empty_before = Group::load(self.ctrl(index_before)).match_empty();
            let empty_after = Group::load(self.ctrl(index)).match_empty();

            let ctrl = if empty_before.leading_zeros() + empty_after.trailing_zeros() >= Group::WIDTH {
                Tag::DELETED
            } else {
                self.growth_left += 1;
                Tag::EMPTY
            };

            self.set_ctrl(index, ctrl);
            self.items -= 1;
        }
    }

    let size = 8; // Adjust as necessary for test
    let mut table = RawTableInner::new(size);

    unsafe {
        // First erase to set DELETED state
        table.erase(1); // Erase second index

        // Try erasing again under the same conditions to increase growth_left
        table.erase(2); // Erase third index
        // Check both control bytes now have altered states
        assert_eq!(table.ctrl_data[1], Tag::DELETED as u8);
        assert_eq!(table.ctrl_data[2], Tag::DELETED as u8);
        assert_eq!(table.items, size - 2);
        assert_eq!(table.growth_left, 1);
    }
}

