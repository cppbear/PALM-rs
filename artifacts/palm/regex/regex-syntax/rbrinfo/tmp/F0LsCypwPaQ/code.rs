fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ast::print::Printer;
        Printer::new().print(self, f)
    }