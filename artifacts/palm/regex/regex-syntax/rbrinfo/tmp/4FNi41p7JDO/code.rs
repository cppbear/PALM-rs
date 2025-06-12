fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use hir::print::Printer;
        Printer::new().print(self, f)
    }