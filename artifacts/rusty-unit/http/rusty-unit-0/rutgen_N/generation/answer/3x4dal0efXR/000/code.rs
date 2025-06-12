// Answer 0

#[derive(Debug, Default)]
struct MockDanger {
    level: DangerLevel,
}

impl MockDanger {
    fn is_yellow(&self) -> bool {
        matches!(self.level, DangerLevel::Yellow)
    }

    fn set_green(&mut self) {
        self.level = DangerLevel::Green;
    }

    fn set_red(&mut self) {
        self.level = DangerLevel::Red;
    }
}

#[derive(Debug, PartialEq)]
enum DangerLevel {
    Green,
    Yellow,
    Red,
}

#[derive(Debug, Default)]
struct Pos;

impl Pos {
    fn none() -> Self {
        Pos
    }
}

#[derive(Debug, Default)]
struct Map {
    entries: Vec<Pos>,
    indices: Box<[Pos]>,
    danger: MockDanger,
}

impl Map {
    fn capacity(&self) -> usize {
        self.indices.len()
    }

    fn try_grow(&mut self, new_cap: usize) -> Result<(), MaxSizeReached> {
        self.indices = vec![Pos::none(); new_cap].into_boxed_slice();
        Ok(())
    }

    fn rebuild(&mut self) {
        // Simulate rebuilding the hash table
    }

    fn try_reserve_one(&mut self) -> Result<(), MaxSizeReached> {
        let len = self.entries.len();

        if self.danger.is_yellow() {
            let load_factor = self.entries.len() as f32 / self.indices.len() as f32;

            if load_factor >= 0.75 {
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
                self.entries = Vec::with_capacity(new_raw_cap);
            } else {
                let raw_cap = self.indices.len();
                self.try_grow(raw_cap << 1)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct MaxSizeReached;

#[test]
fn test_try_reserve_one_when_empty() {
    let mut map = Map::default();
    assert_eq!(map.try_reserve_one(), Ok(()));
    assert_eq!(map.capacity(), 8);
}

#[test]
fn test_try_reserve_one_when_full_and_danger_yellow() {
    let mut map = Map {
        entries: vec![Pos::none(); 5],
        indices: vec![Pos::none(); 8].into_boxed_slice(),
        danger: MockDanger { level: DangerLevel::Yellow },
    };
    let result = map.try_reserve_one();
    assert_eq!(result, Ok(()));
    assert_eq!(map.capacity(), 16);
}

#[test]
fn test_try_reserve_one_when_full_and_danger_green() {
    let mut map = Map {
        entries: vec![Pos::none(); 8],
        indices: vec![Pos::none(); 8].into_boxed_slice(),
        danger: MockDanger { level: DangerLevel::Green },
    };
    let result = map.try_reserve_one();
    assert_eq!(result, Ok(()));
    assert_eq!(map.capacity(), 16);
}

#[test]
fn test_try_reserve_one_when_danger_red_and_needs_rebuild() {
    let mut map = Map {
        entries: vec![Pos::none(); 5],
        indices: vec![Pos::none(); 8].into_boxed_slice(),
        danger: MockDanger { level: DangerLevel::Red },
    };
    map.try_reserve_one().unwrap();
    assert_eq!(map.danger.level, DangerLevel::Red);
    assert!(map.entries.is_empty());
}

