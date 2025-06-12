// Answer 0

fn test_try_reserve_one_empty_and_green() {
    struct TestDanger {
        level: DangerLevel,
    }

    impl TestDanger {
        fn is_yellow(&self) -> bool {
            self.level == DangerLevel::Yellow
        }

        fn set_green(&mut self) {
            self.level = DangerLevel::Green;
        }

        fn set_red(&mut self) {
            self.level = DangerLevel::Red;
        }
    }

    #[derive(Clone, Copy)]
    enum DangerLevel {
        Green,
        Yellow,
        Red,
    }

    struct TestMap {
        entries: Vec<usize>,
        indices: Box<[Pos]>,
        capacity: usize,
        danger: TestDanger,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
                indices: vec![Pos::none(); 8].into_boxed_slice(),
                capacity: 0,
                danger: TestDanger { level: DangerLevel::Green },
            }
        }

        fn try_grow(&mut self, new_cap: usize) -> Result<(), MaxSizeReached> {
            self.indices = vec![Pos::none(); new_cap].into_boxed_slice();
            self.capacity = new_cap;
            Ok(())
        }

        fn capacity(&self) -> usize {
            self.capacity
        }

        fn try_reserve_one(&mut self) -> Result<(), MaxSizeReached> {
            let len = self.entries.len();

            if self.danger.is_yellow() {
                let load_factor = self.entries.len() as f32 / self.indices.len() as f32;
                if load_factor >= LOAD_FACTOR_THRESHOLD {
                    self.danger.set_green();
                    let new_cap = self.indices.len() * 2;
                    self.try_grow(new_cap)?;
                } else {
                    self.danger.set_red();
                    for index in self.indices.iter_mut() {
                        *index = Pos::none();
                    }
                    self.rebuild();
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

        fn rebuild(&mut self) {
            // Placeholder for rebuild logic
        }
    }

    #[derive(Clone, Copy)]
    struct Pos;

    impl Pos {
        fn none() -> Self {
            Pos {}
        }
    }

    struct MaxSizeReached;

    const LOAD_FACTOR_THRESHOLD: f32 = 0.75;
    
    fn usable_capacity(capacity: usize) -> usize {
        capacity / 2
    }

    let mut map = TestMap::new();
    map.try_reserve_one().unwrap();
}

fn test_try_reserve_one_non_empty_and_green() {
    struct TestDanger {
        level: DangerLevel,
    }

    impl TestDanger {
        fn is_yellow(&self) -> bool {
            self.level == DangerLevel::Yellow
        }

        fn set_green(&mut self) {
            self.level = DangerLevel::Green;
        }

        fn set_red(&mut self) {
            self.level = DangerLevel::Red;
        }
    }

    #[derive(Clone, Copy)]
    enum DangerLevel {
        Green,
        Yellow,
        Red,
    }

    struct TestMap {
        entries: Vec<usize>,
        indices: Box<[Pos]>,
        capacity: usize,
        danger: TestDanger,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: Vec::with_capacity(2),
                indices: vec![Pos::none(); 8].into_boxed_slice(),
                capacity: 8,
                danger: TestDanger { level: DangerLevel::Green },
            }
        }

        fn try_grow(&mut self, new_cap: usize) -> Result<(), MaxSizeReached> {
            self.indices = vec![Pos::none(); new_cap].into_boxed_slice();
            self.capacity = new_cap;
            Ok(())
        }

        fn capacity(&self) -> usize {
            self.capacity
        }

        fn try_reserve_one(&mut self) -> Result<(), MaxSizeReached> {
            let len = self.entries.len();
            if self.danger.is_yellow() {
                let load_factor = self.entries.len() as f32 / self.indices.len() as f32;
                if load_factor >= LOAD_FACTOR_THRESHOLD {
                    self.danger.set_green();
                    let new_cap = self.indices.len() * 2;
                    self.try_grow(new_cap)?;
                } else {
                    self.danger.set_red();
                    for index in self.indices.iter_mut() {
                        *index = Pos::none();
                    }
                    self.rebuild();
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

        fn rebuild(&mut self) {
            // Placeholder for rebuild logic
        }
    }

    #[derive(Clone, Copy)]
    struct Pos;

    impl Pos {
        fn none() -> Self {
            Pos {}
        }
    }

    struct MaxSizeReached;

    const LOAD_FACTOR_THRESHOLD: f32 = 0.75;

    fn usable_capacity(capacity: usize) -> usize {
        capacity / 2
    }

    let mut map = TestMap::new();
    map.entries.push(1);
    map.try_reserve_one().unwrap();
}

