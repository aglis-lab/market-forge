use crate::order::{OrderSide, Price, Quantity};

#[derive(Debug)]
pub struct OrderMatch {
    pub order_side: OrderSide,
    pub price: Price,
    pub quantity: Quantity,

    pub match_from_id: u64,
    pub match_to_id: u64,
}

#[derive(Debug, PartialEq)]
pub enum MatchPriceType {
    Partial,
    Full,
}

#[derive(Debug)]
pub struct MatchPrices {
    prices: Vec<Price>,
    match_type: MatchPriceType,
}

impl MatchPrices {
    pub fn new() -> Self {
        return MatchPrices {
            prices: Vec::with_capacity(10),
            match_type: MatchPriceType::Full,
        };
    }

    // pub fn new(prices: Vec<Price>, match_type: MatchPriceType) -> Self {
    //     return MatchPrices { prices, match_type };
    // }

    pub fn set_partially_match(&mut self) {
        self.match_type = MatchPriceType::Partial;
    }

    pub fn is_partially_fill(&self) -> bool {
        return self.match_type == MatchPriceType::Partial;
    }

    pub fn is_fully_fill(&self) -> bool {
        return self.match_type == MatchPriceType::Full;
    }

    pub fn get_top_prices(&self) -> &Vec<Price> {
        return &self.prices;
    }

    pub fn get_top_prices_mut(&mut self) -> &mut Vec<Price> {
        return &mut self.prices;
    }
}
