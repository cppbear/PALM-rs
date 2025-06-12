fn follow_epsilons(
        &mut self,
        ip: InstPtr,
        q: &mut SparseSet,
        flags: EmptyFlags,
    ) {
        use prog::Inst::*;
        use prog::EmptyLook::*;

        // We need to traverse the NFA to follow epsilon transitions, so avoid
        // recursion with an explicit stack.
        self.cache.stack.push(ip);
        while let Some(mut ip) = self.cache.stack.pop() {
            // Try to munch through as many states as possible without
            // pushes/pops to the stack.
            loop {
                // Don't visit states we've already added.
                if q.contains(ip as usize) {
                    break;
                }
                q.insert(ip as usize);
                match self.prog[ip as usize] {
                    Char(_) | Ranges(_) => unreachable!(),
                    Match(_) | Bytes(_) => {
                        break;
                    }
                    EmptyLook(ref inst) => {
                        // Only follow empty assertion states if our flags
                        // satisfy the assertion.
                        match inst.look {
                            StartLine if flags.start_line => {
                                ip = inst.goto as InstPtr;
                            }
                            EndLine if flags.end_line => {
                                ip = inst.goto as InstPtr;
                            }
                            StartText if flags.start => {
                                ip = inst.goto as InstPtr;
                            }
                            EndText if flags.end => {
                                ip = inst.goto as InstPtr;
                            }
                            WordBoundaryAscii if flags.word_boundary => {
                                ip = inst.goto as InstPtr;
                            }
                            NotWordBoundaryAscii if flags.not_word_boundary => {
                                ip = inst.goto as InstPtr;
                            }
                            WordBoundary if flags.word_boundary => {
                                ip = inst.goto as InstPtr;
                            }
                            NotWordBoundary if flags.not_word_boundary => {
                                ip = inst.goto as InstPtr;
                            }
                            StartLine | EndLine | StartText | EndText
                            | WordBoundaryAscii | NotWordBoundaryAscii
                            | WordBoundary | NotWordBoundary => {
                                break;
                            }
                        }
                    }
                    Save(ref inst) => {
                        ip = inst.goto as InstPtr;
                    }
                    Split(ref inst) => {
                        self.cache.stack.push(inst.goto2 as InstPtr);
                        ip = inst.goto1 as InstPtr;
                    }
                }
            }
        }
    }