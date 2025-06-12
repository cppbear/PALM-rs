// Answer 0

#[derive(Debug)]
struct Character {
    len: usize,
}

impl Character {
    pub fn new(len: usize) -> Self {
        Character { len }
    }
}

#[test]
fn test_character_len() {
    let char1 = Character::new(1);
    let char2 = Character::new(3);
    let char3 = Character::new(0);
    
    assert_eq!(char1.len(), 1);
    assert_eq!(char2.len(), 3);
    assert_eq!(char3.len(), 0);
}

#[test]
fn test_character_len_boundary() {
    let char_max = Character::new(usize::MAX);
    
    assert_eq!(char_max.len(), usize::MAX);
}

