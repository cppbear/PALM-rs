// Answer 0

#[derive(Default)]
struct Reader {
    data: Vec<char>,
    index: usize,
}

impl Reader {
    fn discard(&mut self) {
        if self.index < self.data.len() {
            self.index += 1;
        } else {
            panic!("Index out of bounds");
        }
    }
}

struct TestStruct {
    read: Reader,
}

impl TestStruct {
    fn eat_char(&mut self) {
        self.read.discard();
    }
}

#[test]
fn test_eat_char_with_valid_index() {
    let mut test_struct = TestStruct {
        read: Reader {
            data: vec!['a', 'b', 'c'],
            index: 0,
        },
    };
    test_struct.eat_char();
    assert_eq!(test_struct.read.index, 1);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_eat_char_with_out_of_bounds_index() {
    let mut test_struct = TestStruct {
        read: Reader {
            data: vec!['a'],
            index: 1,
        },
    };
    test_struct.eat_char();
} 

#[test]
fn test_eat_char_multiple_calls() {
    let mut test_struct = TestStruct {
        read: Reader {
            data: vec!['x', 'y', 'z'],
            index: 0,
        },
    };
    test_struct.eat_char();
    test_struct.eat_char();
    
    assert_eq!(test_struct.read.index, 2);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_eat_char_on_empty_data() {
    let mut test_struct = TestStruct {
        read: Reader {
            data: vec![],
            index: 0,
        },
    };
    test_struct.eat_char();
}

