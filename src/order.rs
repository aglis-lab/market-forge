use std::collections::VecDeque;

pub type Price = u128;
pub type Quantity = u128;

#[derive(PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

pub struct Order {
    id: u64,
    price: Price,
    quantity: Quantity,
    order_side: OrderSide,
}

pub struct Orders {
    items: VecDeque<Order>,
    length: u32,
    total_quantity: Quantity,
}

impl Clone for OrderSide {
    fn clone(&self) -> Self {
        match self {
            Self::Buy => Self::Buy,
            Self::Sell => Self::Sell,
        }
    }
}

impl Clone for Order {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            price: self.price.clone(),
            quantity: self.quantity.clone(),
            order_side: self.order_side.clone(),
        }
    }
}

impl Order {
    pub fn new(order_side: OrderSide, id: u64, price: Price, quantity: Quantity) -> Self {
        return Order {
            id,
            price,
            quantity,
            order_side,
        };
    }

    pub fn id(&self) -> u64 {
        return self.id;
    }

    pub fn price(&self) -> Price {
        return self.price;
    }

    pub fn quantity(&self) -> Quantity {
        return self.quantity;
    }

    pub fn order_side(&self) -> &OrderSide {
        return &self.order_side;
    }

    pub fn is_buy(&self) -> bool {
        return self.order_side == OrderSide::Buy;
    }

    pub fn is_sell(&self) -> bool {
        return self.order_side == OrderSide::Sell;
    }

    pub fn match_price(&self, other_price: Price) -> bool {
        if self.is_buy() && other_price <= self.price() {
            return true;
        }

        if self.is_sell() && other_price >= self.price() {
            return true;
        }

        return false;
    }
}

impl Orders {
    pub fn new() -> Self {
        return Orders {
            items: VecDeque::new(),
            length: 0,
            total_quantity: 0,
        };
    }

    pub fn add(&mut self, new_oder: Order) {
        let quantity = new_oder.quantity();

        self.items.push_back(new_oder);
        self.length += 1;
        self.total_quantity += quantity;
    }

    pub fn len(&self) -> u32 {
        self.length
    }

    pub fn total_quantity(&self) -> Quantity {
        self.total_quantity
    }

    pub fn items(&self) -> &VecDeque<Order> {
        &self.items
    }
}
