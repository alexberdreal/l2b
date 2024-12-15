use std::{
    collections::BTreeMap,
    ops::AddAssign
};
use std::ops::SubAssign;

pub(crate) use protocol::prelude::*;

struct OrderBook {
    bids: BTreeMap<Price, Amount>,
    asks: BTreeMap<Price, Amount>,
}

impl OrderBook {
    fn new() -> Self {
        Self {
            bids: Default::default(),
            asks: Default::default(),
        }
    }
    fn add(&mut self, price: Price, amount: Amount, side: Side) {
        match side {
            Side::Buy => {
                self.bids.entry(price).or_insert(amt!(0)).add_assign(amount);
            }
            Side::Sell => {
                self.asks.entry(price).or_insert(amt!(0)).add_assign(amount);
            }
        }
    }

    fn remove(&mut self, price: Price, amount: Amount, side: Side) -> Result<(), OrderBookErr> {
        let add_level = |levels: &mut BTreeMap<Price, Amount>| -> Result<(), OrderBookErr> {
            let lvl = levels
                .get_mut(&price)
                .ok_or(OrderBookErr {
                    reason: "no price level found",
                    price: Some(price),
                    amount: Some(amount),
                    side,
                })?;

            let diff = lvl
                .checked_sub(amount)
                .ok_or(OrderBookErr {
                    reason: "checked_sub overflow",
                    price: Some(price),
                    amount: Some(amount),
                    side,
                })?;

            lvl.sub_assign(diff);

            Ok(())
        };

        match side {
            Side::Buy => add_level(&mut self.bids),
            Side::Sell => add_level(&mut self.asks),
        }
    }

    fn get_amount(&self, price: Price, side: Side) -> Result<Amount, OrderBookErr> {
        let get_amount = |levels:  &BTreeMap<Price, Amount>| -> Result<Amount, OrderBookErr> {
            return levels.get(&price).copied().ok_or(OrderBookErr {
                reason: "get amount overflow",
                price: Some(price),
                amount: None,
                side,
            });
        };

        match side {
            Side::Buy => get_amount(&self.bids),
            Side::Sell => get_amount(&self.asks)
        }
    }

    #[cfg(test)]
    fn get_bids(&self) -> Vec<(Price, Amount)> {
        self.bids.iter().rev().map(|(p, a)|(*p, *a)).collect()
    }

    #[cfg(test)]
    fn get_asks(&self) -> Vec<(Price, Amount)> {
        self.asks.iter().map(|(p, a)|(*p, *a)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_operations_asks() {
        let mut ob = OrderBook::new();
        let mut price = price!(0);
        let amount = amt!(10);
        for _ in 0..100 {
            price.add_assign(price!(1));
            ob.add(price, amount, Side::Buy);
        }
        price.add_assign(price!(5));
        for _ in 0..100 {
            price.add_assign(price!(1));
            ob.add(price, amount, Side::Sell);
        }
        assert_eq!(ob.get_asks().len(), 100);
        assert_eq!(ob.get_bids().len(), 100);
        assert!(ob.remove(price!(150), amt!(5), Side::Sell).is_ok());
        assert!(ob.get_amount(price!(150), Side::Sell).is_ok());
        assert_eq!(ob.get_amount(price!(150), Side::Sell).expect("checked"), amt!(5));
        assert!(ob.remove(price!(70), amt!(5), Side::Sell).is_err());
    }
}
