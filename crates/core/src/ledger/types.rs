pub struct SimpleLedger {
    balance: u64,
    limit: u64,
}

pub struct DualSimpleLedger {
    ours: SimpleLedger,
    theirs: SimpleLedger,
    // pending_tx
}
