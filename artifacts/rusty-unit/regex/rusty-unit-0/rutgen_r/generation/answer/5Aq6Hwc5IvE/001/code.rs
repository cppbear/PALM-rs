// Answer 0

#[test]
fn test_pop_class_with_empty_stack_panics() {
    struct Parser {
        stack_class: Vec<ClassState>,
        pos: usize,
        char: char,
    }

    impl Parser {
        fn new() -> Self {
            Self {
                stack_class: Vec::new(),
                pos: 0,
                char: ']',
            }
        }

        fn char(&self) -> char {
            self.char
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pop_class_op(&self, _item: ClassSet) -> ClassSetUnion {
            // Assume we return a default for the sake of this test
            ClassSetUnion {}
        }

        fn parser(&self) -> &Self {
            self
        }

        fn pop_class(&mut self, nested_union: ClassSetUnion) -> Result<Either<ClassSetUnion, Class>, String> {
            assert_eq!(self.char(), ']');

            let item = ClassSet::Item(nested_union.into_item());
            let prevset = self.pop_class_op(item);
            match self.stack_class.pop() {
                None => panic!("unexpected empty character class stack"),
                Some(ClassState::Open { mut union, mut set }) => {
                    self.bump();
                    set.span.end = self.pos();
                    set.kind = prevset;
                    Ok(Either::Left(union))
                },
                _ => panic!("unexpected ClassState::Op"),
            }
        }
    }

    let mut parser = Parser::new();
    let nested_union = ClassSetUnion {};
    let result = std::panic::catch_unwind(|| {
        parser.pop_class(nested_union).unwrap();
    });
    assert!(result.is_err());
}

#[test]
fn test_pop_class_with_op_state_panics() {
    struct Parser {
        stack_class: Vec<ClassState>,
        pos: usize,
        char: char,
    }

    impl Parser {
        fn new() -> Self {
            Self {
                stack_class: vec![ClassState::Op {}],
                pos: 0,
                char: ']',
            }
        }

        fn char(&self) -> char {
            self.char
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pop_class_op(&self, _item: ClassSet) -> ClassSetUnion {
            ClassSetUnion {}
        }

        fn parser(&self) -> &Self {
            self
        }

        fn pop_class(&mut self, nested_union: ClassSetUnion) -> Result<Either<ClassSetUnion, Class>, String> {
            assert_eq!(self.char(), ']');

            let item = ClassSet::Item(nested_union.into_item());
            let prevset = self.pop_class_op(item);
            match self.stack_class.pop() {
                Some(ClassState::Op { .. }) => panic!("unexpected ClassState::Op"),
                Some(ClassState::Open { mut union, mut set }) => {
                    self.bump();
                    set.span.end = self.pos();
                    set.kind = prevset;
                    Ok(Either::Left(union))
                },
                None => panic!("unexpected empty character class stack"),
            }
        }
    }

    let mut parser = Parser::new();
    let nested_union = ClassSetUnion {};
    let result = std::panic::catch_unwind(|| {
        parser.pop_class(nested_union).unwrap();
    });
    assert!(result.is_err());
}

#[test]
fn test_pop_class_with_open_state() {
    struct Parser {
        stack_class: Vec<ClassState>,
        pos: usize,
        char: char,
    }

    impl Parser {
        fn new() -> Self {
            Self {
                stack_class: vec![ClassState::Open { union: ClassSetUnion {}, set: ClassSet::new() }],
                pos: 0,
                char: ']',
            }
        }

        fn char(&self) -> char {
            self.char
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pop_class_op(&self, _item: ClassSet) -> ClassSetUnion {
            ClassSetUnion {}
        }

        fn parser(&self) -> &Self {
            self
        }

        fn pop_class(&mut self, nested_union: ClassSetUnion) -> Result<Either<ClassSetUnion, Class>, String> {
            assert_eq!(self.char(), ']');

            let item = ClassSet::Item(nested_union.into_item());
            let prevset = self.pop_class_op(item);
            match self.stack_class.pop() {
                Some(ClassState::Open { mut union, mut set }) => {
                    self.bump();
                    set.span.end = self.pos();
                    set.kind = prevset;
                    if self.stack_class.is_empty() {
                        Ok(Either::Right(Class::Bracketed(set)))
                    } else {
                        union.push(ClassSetItem::Bracketed(Box::new(set)));
                        Ok(Either::Left(union))
                    }
                },
                None => panic!("unexpected empty character class stack"),
                Some(ClassState::Op { .. }) => panic!("unexpected ClassState::Op"),
            }
        }
    }

    let mut parser = Parser::new();
    let nested_union = ClassSetUnion {};
    let result = parser.pop_class(nested_union).unwrap();
    assert!(matches!(result, Either::Right(_)));
}

