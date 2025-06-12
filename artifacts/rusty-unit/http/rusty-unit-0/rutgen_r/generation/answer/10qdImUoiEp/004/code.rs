// Answer 0

#[derive(Debug)]
struct HeaderName(String);
#[derive(Debug)]
struct HashValue(usize);
#[derive(Debug)]
struct MaxSizeReached;
#[derive(Debug)]
struct Pos {
    index: usize,
    hash: HashValue,
}

impl Pos {
    fn new(index: usize, hash: HashValue) -> Self {
        Pos { index, hash }
    }
}

struct Entry<T> {
    entries: Vec<(HeaderName, T)>,
    indices: Vec<Pos>,
    danger: DangerLevel,
}

struct DangerLevel {
    level: String,
}

impl DangerLevel {
    fn set_yellow(&mut self) {
        self.level = "yellow".to_string();
    }
}

impl<T> Entry<T> {
    fn new() -> Self {
        Entry {
            entries: Vec::new(),
            indices: Vec::new(),
            danger: DangerLevel { level: "safe".to_string() },
        }
    }

    fn try_insert_entry(&mut self, hash: HashValue, key: HeaderName, value: T) -> Result<(), MaxSizeReached> {
        self.entries.push((key, value));
        Ok(())
    }

    fn try_insert_phase_two(&mut self, key: HeaderName, value: T, hash: HashValue, probe: usize, danger: bool) -> Result<usize, MaxSizeReached> {
        let index = self.entries.len();
        self.try_insert_entry(hash, key, value)?;

        let num_displaced = self.do_insert_phase_two(probe, Pos::new(index, hash));

        if danger || num_displaced >= 3 { // Assuming DISPLACEMENT_THRESHOLD is 3 for this example
            self.danger.set_yellow();
        }

        Ok(index)
    }

    fn do_insert_phase_two(&mut self, probe: usize, pos: Pos) -> usize {
        // Simulating displacement, return a number less than DISPLACEMENT_THRESHOLD for testing
        self.indices.push(pos);
        2 // Simulated number of displaced entries
    }
}

#[test]
fn test_try_insert_phase_two_success() {
    let mut entry: Entry<i32> = Entry::new();
    let key = HeaderName("Test-Key".to_string());
    let value = 42;
    let hash = HashValue(1);
    let probe = 0;
    let danger = false;

    let result = entry.try_insert_phase_two(key, value, hash, probe, danger);

    assert_eq!(result, Ok(0)); // We expect the return value to be Ok(0) as we inserted the first entry
}

#[test]
fn test_try_insert_phase_two_with_no_displacement() {
    let mut entry: Entry<i32> = Entry::new();
    let key = HeaderName("Another-Key".to_string());
    let value = 99;
    let hash = HashValue(2);
    let probe = 0;
    let danger = false;

    entry.try_insert_phase_two(HeaderName("Test-Key".to_string()), 42, HashValue(1), probe, false).unwrap(); // Insert first entry

    let result = entry.try_insert_phase_two(key, value, hash, probe, danger);

    assert_eq!(result, Ok(1)); // We expect the index to be 1 for the second entry
}

