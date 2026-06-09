pub struct MatchingPoolConfig {
    pub symbols: std::vec::Vec<Symbol>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub symbol: String,
    pub slot_idx: usize,
}
