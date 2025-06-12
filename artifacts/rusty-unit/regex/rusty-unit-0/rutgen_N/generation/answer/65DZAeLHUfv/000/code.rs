// Answer 0

#[test]
fn test_case_fold_simple_with_lowercase_range() {
    struct Set {
        ranges: Vec<(char, char)>,
    }

    impl Set {
        fn new() -> Self {
            Set { ranges: vec![] }
        }

        fn add_range(&mut self, start: char, end: char) {
            self.ranges.push((start, end));
        }

        fn case_fold_simple(&mut self) {
            let mut new_ranges = vec![];
            for &(start, end) in &self.ranges {
                new_ranges.push((start, end));
                if start >= 'a' && end <= 'z' {
                    new_ranges.push(('A', (end as u8 - ('a' as u8 - 'A' as u8)) as char));
                }
            }
            self.ranges = new_ranges;
        }
    }

    struct MyStruct {
        set: Set,
    }

    impl MyStruct {
        fn new() -> Self {
            MyStruct { set: Set::new() }
        }

        fn case_fold_simple(&mut self) {
            self.set.case_fold_simple();
        }
    }

    let mut my_struct = MyStruct::new();
    my_struct.set.add_range('a', 'z');
    my_struct.case_fold_simple();

    assert_eq!(my_struct.set.ranges, vec![('a', 'z'), ('A', 'Z')]);
}

#[test]
fn test_case_fold_simple_with_mixed_range() {
    struct Set {
        ranges: Vec<(char, char)>,
    }

    impl Set {
        fn new() -> Self {
            Set { ranges: vec![] }
        }

        fn add_range(&mut self, start: char, end: char) {
            self.ranges.push((start, end));
        }

        fn case_fold_simple(&mut self) {
            let mut new_ranges = vec![];
            for &(start, end) in &self.ranges {
                new_ranges.push((start, end));
                if start >= 'a' && end <= 'z' {
                    new_ranges.push(('A', (end as u8 - ('a' as u8 - 'A' as u8)) as char));
                }
            }
            self.ranges = new_ranges;
        }
    }

    struct MyStruct {
        set: Set,
    }

    impl MyStruct {
        fn new() -> Self {
            MyStruct { set: Set::new() }
        }

        fn case_fold_simple(&mut self) {
            self.set.case_fold_simple();
        }
    }

    let mut my_struct = MyStruct::new();
    my_struct.set.add_range('a', 'c');
    my_struct.set.add_range('d', 'g');
    my_struct.case_fold_simple();

    assert_eq!(my_struct.set.ranges, vec![('a', 'c'), ('A', 'C'), ('d', 'g')]);
}

#[test]
fn test_case_fold_simple_with_empty_set() {
    struct Set {
        ranges: Vec<(char, char)>,
    }

    impl Set {
        fn new() -> Self {
            Set { ranges: vec![] }
        }

        fn case_fold_simple(&mut self) {
            let mut new_ranges = vec![];
            for &(start, end) in &self.ranges {
                new_ranges.push((start, end));
                if start >= 'a' && end <= 'z' {
                    new_ranges.push(('A', (end as u8 - ('a' as u8 - 'A' as u8)) as char));
                }
            }
            self.ranges = new_ranges;
        }
    }

    struct MyStruct {
        set: Set,
    }

    impl MyStruct {
        fn new() -> Self {
            MyStruct { set: Set::new() }
        }

        fn case_fold_simple(&mut self) {
            self.set.case_fold_simple();
        }
    }

    let mut my_struct = MyStruct::new();
    my_struct.case_fold_simple();

    assert_eq!(my_struct.set.ranges, vec![]);
}

