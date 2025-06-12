// Answer 0

#[test]
fn test_try_reserve_one_with_capacity() {
    struct Pos;

    impl Pos {
        fn none() -> Self {
            Pos
        }
    }

    struct Danger {
        is_yellow: bool,
    }

    impl Danger {
        fn is_yellow(&self) -> bool {
            self.is_yellow
        }

        fn set_green(&mut self) {
            self.is_yellow = false;
        }

        fn set_red(&mut self) {
            self.is_yellow = true;
        }
    }

    struct TestStruct {
        danger: Danger,
        entries: Vec<i32>,
        indices: Box<[Pos]>,
    }

    impl TestStruct {
        fn capacity(&self) -> usize {
            self.indices.len()
        }

        fn try_grow(&mut self, _new_cap: usize) -> Result<(), ()> {
            Ok(())
        }

        fn try_reserve_one(&mut self) -> Result<(), ()> {
            let len = self.entries.len();

            if self.danger.is_yellow() {
                let load_factor = self.entries.len() as f32 / self.indices.len() as f32;

                if load_factor >= 1.0 {
                    self.danger.set_green();
                    let new_cap = self.indices.len() * 2;
                    self.try_grow(new_cap)?;
                } else {
                    self.danger.set_red();
                    for index in self.indices.iter_mut() {
                        *index = Pos::none();
                    }
                    // Call to rebuild, omitted for simplicity
                }
            } else if len == self.capacity() {
                if len == 0 {
                    let new_raw_cap = 8;
                    self.indices = vec![Pos::none(); new_raw_cap].into_boxed_slice();
                    self.entries = Vec::with_capacity(usable_capacity(new_raw_cap));
                } else {
                    let raw_cap = self.indices.len();
                    self.try_grow(raw_cap << 1)?;
                }
            }

            Ok(())
        }
    }

    fn usable_capacity(cap: usize) -> usize {
        cap
    }

    let mut my_struct = TestStruct {
        danger: Danger { is_yellow: false },
        entries: vec![1], // Ensure len is not zero
        indices: vec![Pos::none(); 4].into_boxed_slice(), // Set the capacity
    };

    assert_eq!(my_struct.try_reserve_one(), Ok(()));
}

