// Answer 0

#[test]
fn test_is_all_ascii_empty_class() {
    struct CharacterClass {
        set: std::ops::RangeInclusive<u8>,
    }
    
    impl CharacterClass {
        pub fn intervals(&self) -> Vec<std::ops::RangeInclusive<u8>> {
            vec![self.set.clone()]
        }
        
        pub fn new(set: std::ops::RangeInclusive<u8>) -> Self {
            CharacterClass { set }
        }
    }
    
    let class = CharacterClass::new(0..=0);
    assert!(class.is_all_ascii());
}

#[test]
fn test_is_all_ascii_only_ascii() {
    struct CharacterClass {
        set: std::ops::RangeInclusive<u8>,
    }
    
    impl CharacterClass {
        pub fn intervals(&self) -> Vec<std::ops::RangeInclusive<u8>> {
            vec![self.set.clone()]
        }
        
        pub fn new(set: std::ops::RangeInclusive<u8>) -> Self {
            CharacterClass { set }
        }
    }
    
    let class = CharacterClass::new(0..=127);
    assert!(class.is_all_ascii());
}

#[test]
fn test_is_all_ascii_non_ascii() {
    struct CharacterClass {
        set: std::ops::RangeInclusive<u8>,
    }
    
    impl CharacterClass {
        pub fn intervals(&self) -> Vec<std::ops::RangeInclusive<u8>> {
            vec![self.set.clone()]
        }
        
        pub fn new(set: std::ops::RangeInclusive<u8>) -> Self {
            CharacterClass { set }
        }
    }
    
    let class = CharacterClass::new(128..=255);
    assert!(!class.is_all_ascii());
}

#[test]
fn test_is_all_ascii_mixed() {
    struct CharacterClass {
        set: std::ops::RangeInclusive<u8>,
    }
    
    impl CharacterClass {
        pub fn intervals(&self) -> Vec<std::ops::RangeInclusive<u8>> {
            vec![self.set.clone()]
        }
        
        pub fn new(set: std::ops::RangeInclusive<u8>) -> Self {
            CharacterClass { set }
        }
    }
    
    let class = CharacterClass::new(0..=200);
    assert!(!class.is_all_ascii());
}

#[test]
fn test_is_all_ascii_single_ascii() {
    struct CharacterClass {
        set: std::ops::RangeInclusive<u8>,
    }
    
    impl CharacterClass {
        pub fn intervals(&self) -> Vec<std::ops::RangeInclusive<u8>> {
            vec![self.set.clone()]
        }
        
        pub fn new(set: std::ops::RangeInclusive<u8>) -> Self {
            CharacterClass { set }
        }
    }
    
    let class = CharacterClass::new(64..=64);
    assert!(class.is_all_ascii());
}

#[test]
fn test_is_all_ascii_boundary() {
    struct CharacterClass {
        set: std::ops::RangeInclusive<u8>,
    }
    
    impl CharacterClass {
        pub fn intervals(&self) -> Vec<std::ops::RangeInclusive<u8>> {
            vec![self.set.clone()]
        }
        
        pub fn new(set: std::ops::RangeInclusive<u8>) -> Self {
            CharacterClass { set }
        }
    }
    
    let class = CharacterClass::new(0..=255);
    assert!(!class.is_all_ascii());
}

