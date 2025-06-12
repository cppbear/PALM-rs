// Answer 0

#[test]
fn test_split_at_mut_valid() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: "one" },
            Bucket { hash: 0, key: 2, value: "two" },
            Bucket { hash: 0, key: 3, value: "three" },
        ],
    };
    
    let (first_half, second_half) = slice.split_at_mut(2);
    
    assert_eq!(first_half.len(), 2);
    assert_eq!(second_half.len(), 1);
    assert_eq!(first_half.get_index(0), Some((&1, &"one")));
    assert_eq!(first_half.get_index(1), Some((&2, &"two")));
    assert_eq!(second_half.get_index(0), Some((&3, &"three")));
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_split_at_mut_panic() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: "one" },
            Bucket { hash: 0, key: 2, value: "two" },
            Bucket { hash: 0, key: 3, value: "three" },
        ],
    };
    
    let _ = slice.split_at_mut(4);
} 

#[test]
fn test_split_at_mut_empty() {
    let mut slice = Slice {
        entries: [],
    };
    
    let (first_half, second_half) = slice.split_at_mut(0);
    
    assert_eq!(first_half.len(), 0);
    assert_eq!(second_half.len(), 0);
}

