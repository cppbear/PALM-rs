pub fn searcher_str(&self) -> ExecNoSyncStr {
        ExecNoSyncStr(self.searcher())
    }