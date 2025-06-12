pub fn build(&self) -> Parser {
        Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: self.nest_limit,
            octal: self.octal,
            initial_ignore_whitespace: self.ignore_whitespace,
            ignore_whitespace: Cell::new(self.ignore_whitespace),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        }
    }