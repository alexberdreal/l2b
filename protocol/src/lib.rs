use std::fmt::{Debug, Formatter};

pub use derive_more::Display;
use rust_decimal::Decimal;

pub type Price = Decimal;
pub type Amount = Decimal;


// TODO: figure out how to do operations without deref for Price/Amount represented as structs

// macro_rules! impl_deref {
//     ($t:ty) => {
//     impl Deref for $t {
//     type Target = Decimal;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//     };
// }
//
// impl_deref!(Price);
// impl_deref!(Amount);

pub struct OrderBookErr {
    pub reason: &'static str,
    pub price: Option<Price>,
    pub amount: Option<Amount>,
    pub side: Side,
}

impl Debug for OrderBookErr {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(_f, "{}", format_args!(
            "order book error; reason: {}; price: {:?}; amount: {:?}; side: {}",
            self.reason, self.price, self.amount, self.side
        ))
    }
}

#[macro_export]
macro_rules! price {
    ($val:literal) => {
        $crate::_private::rust_decimal_macros::dec!($val) as Price
    };
}

#[macro_export]
macro_rules! amt {
    ($val:literal) => {
        $crate::_private::rust_decimal_macros::dec!($val) as Amount
    };
}

#[derive(Debug, Display, PartialEq, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

#[doc(hidden)]
pub mod _private {
    pub use rust_decimal_macros;
}

pub mod prelude {
    pub use crate::{amt, price, Price, Amount, Side, OrderBookErr};
    pub use rust_decimal::prelude::*;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tst() {
        assert!(amt!(1) > amt!(0))
    }
}
