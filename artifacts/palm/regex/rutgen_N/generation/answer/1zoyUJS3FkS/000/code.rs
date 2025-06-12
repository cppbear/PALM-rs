// Answer 0

#[test]
fn test_new_transitions() {
    struct Transitions {
        table: Vec<usize>,
        num_byte_classes: usize,
    }

    fn new(num_byte_classes: usize) -> Transitions {
        Transitions {
            table: vec![],
            num_byte_classes,
        }
    }

    let transitions1 = new(1);
    assert_eq!(transitions1.num_byte_classes, 1);
    assert!(transitions1.table.is_empty());

    let transitions2 = new(5);
    assert_eq!(transitions2.num_byte_classes, 5);
    assert!(transitions2.table.is_empty());

    let transitions3 = new(0);
    assert_eq!(transitions3.num_byte_classes, 0);
    assert!(transitions3.table.is_empty());
}

