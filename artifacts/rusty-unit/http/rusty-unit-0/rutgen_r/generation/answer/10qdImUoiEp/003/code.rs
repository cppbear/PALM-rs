// Answer 0

#[derive(Debug)]
struct HeaderName(String);

#[derive(Debug)]
struct HashValue(u64);

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

struct TestStruct {
    entries: Vec<(HeaderName, usize)>, // Assuming value type is usize for this example
    indices: Vec<Pos>,
    danger: DangerLevel,
}

impl TestStruct {
    fn new() -> Self {
        TestStruct {
            entries: Vec::new(),
            indices: Vec::new(),
            danger: DangerLevel::green(),
        }
    }
    
    fn try_insert_entry(&mut self, _hash: HashValue, key: HeaderName, value: usize) -> Result<(), MaxSizeReached> {
        // Simulated entry insertion
        self.entries.push((key, value));
        Ok(())
    }
}

#[derive(Debug)]
struct DangerLevel {
    level: String,
}

impl DangerLevel {
    fn green() -> Self {
        DangerLevel { level: "green".to_string() }
    }
    
    fn set_yellow(&mut self) {
        self.level = "yellow".to_string();
    }
}

const DISPLACEMENT_THRESHOLD: usize = 5;

fn do_insert_phase_two(indices: &mut Vec<Pos>, _probe: usize, pos: Pos) -> usize {
    indices.push(pos);
    DISPLACEMENT_THRESHOLD // Simulating that we displace exactly DISPLACEMENT_THRESHOLD entries
}

impl TestStruct {
    fn try_insert_phase_two(
        &mut self,
        key: HeaderName,
        value: usize,
        hash: HashValue,
        probe: usize,
        danger: bool,
    ) -> Result<usize, MaxSizeReached> {
        let index = self.entries.len();
        self.try_insert_entry(hash, key, value)?;

        let num_displaced = do_insert_phase_two(&mut self.indices, probe, Pos::new(index, hash));

        if danger || num_displaced >= DISPLACEMENT_THRESHOLD {
            self.danger.set_yellow();
        }

        Ok(index)
    }
}

#[test]
fn test_try_insert_phase_two_success() {
    let mut test_struct = TestStruct::new();
    let key = HeaderName("Test-Key".to_string());
    let value = 42;
    let hash = HashValue(12345);
    let probe = 0;
    let danger = false;

    let result = test_struct.try_insert_phase_two(key, value, hash, probe, danger);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0); // The index of the first insertion
    assert_eq!(test_struct.danger.level, "green"); // Danger level remains green
}

#[test]
fn test_try_insert_phase_two_with_danger() {
    let mut test_struct = TestStruct::new();
    let key = HeaderName("Test-Key".to_string());
    let value = 42;
    let hash = HashValue(12345);
    let probe = 0;
    let danger = false;

    // Fill the entries to approach the displacement threshold
    for i in 0..DISPLACEMENT_THRESHOLD {
        let key = HeaderName(format!("Key-{}", i));
        let _ = test_struct.try_insert_entry(hash, key.clone(), i);
    }

    let result = test_struct.try_insert_phase_two(key, value, hash, probe, danger);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), DISPLACEMENT_THRESHOLD); // The index of this insertion
    assert_eq!(test_struct.danger.level, "yellow"); // Danger level changes to yellow
}

