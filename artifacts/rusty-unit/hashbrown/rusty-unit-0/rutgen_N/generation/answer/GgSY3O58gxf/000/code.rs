// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct HashTable {
        elements: Vec<i32>,
    }

    impl HashTable {
        pub fn new() -> Self {
            HashTable { elements: Vec::new() }
        }

        pub fn len(&self) -> usize {
            self.elements.len()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        pub fn insert(&mut self, value: i32) {
            self.elements.push(value);
        }
    }

    #[test]
    fn test_is_empty_when_empty() {
        let table = HashTable::new();
        assert!(table.is_empty());
    }

    #[test]
    fn test_is_empty_when_not_empty() {
        let mut table = HashTable::new();
        table.insert(1);
        assert!(!table.is_empty());
    }

    #[test]
    fn test_is_empty_after_removal() {
        let mut table = HashTable::new();
        table.insert(1);
        table.elements.pop(); // Simulate removal
        assert!(table.is_empty());
    }
}

