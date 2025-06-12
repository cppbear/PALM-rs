// Answer 0

#[derive(Debug)]
struct Name(String);

trait Hasher {
    fn write(&mut self, bytes: &[u8]);
}

struct SimpleHasher {
    output: Vec<u8>,
}

impl Hasher for SimpleHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.output.extend_from_slice(bytes);
    }
}

impl Name {
    fn new(value: &str) -> Self {
        Name(value.to_string())
    }
    
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write(self.0.as_bytes());
    }
}

#[test]
fn test_hash_empty_name() {
    let name = Name::new("");
    let mut hasher = SimpleHasher { output: Vec::new() };
    name.hash(&mut hasher);
    assert!(hasher.output.is_empty());
}

#[test]
fn test_hash_non_empty_name() {
    let name = Name::new("example");
    let mut hasher = SimpleHasher { output: Vec::new() };
    name.hash(&mut hasher);
    assert_eq!(hasher.output, b"example");
}

