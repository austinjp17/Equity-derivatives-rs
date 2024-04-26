use super::{strategies::*, OptionType};
use crate::{helpers, Error};
use polars::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::BTreeMap;
use std::{
    cmp::{max, min},
    ops::Add,
};
use tracing::{info, trace};

/// Payoff of a particular option
///
/// BTreeMap used for quick key exist hit and ordered keys
#[derive(Debug, Default, Clone)]
pub struct PayoffStruct {
    pub inner: BTreeMap<Decimal, Decimal>,
}
impl PayoffStruct {
    pub fn new(data: BTreeMap<Decimal, Decimal>) -> Self {
        Self { inner: data }
    }
}
// impl core::fmt::Display for PayoffStruct {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:<5} | {:<15}\n", "Price", "Profit")?; // Header
//         write!(f, "-----------\n")?;
//         for (price, profit) in self.price.iter().zip(&self.profit) {
//             write!(f, "{:<5} | {:<15}\n", price, profit)?;
//         }
//         Ok(())
//     }
// }

impl Add for OptionContract {
    type Output = OptionCombo;

    /// Combines two payoff structs
    ///
    /// Uses the outer most bounds of either structs to construct considered price range.
    /// Profits calculated for all 1 step numbers inclusive of start and end
    ///
    /// Accounts for different price range bounds between each struct
    fn add(self, rhs: Self) -> Self::Output {
        assert!(!self.payoff.inner.is_empty());
        assert!(!rhs.payoff.inner.is_empty());

        let mut payoff = PayoffStruct::default();

        let min_price = min(
            self.payoff.inner.first_key_value().expect("Checked").0,
            rhs.payoff.inner.first_key_value().expect("checked").0,
        );

        let max_price = max(
            self.payoff.inner.last_key_value().expect("Checked").0,
            rhs.payoff.inner.last_key_value().expect("Checked").0,
        );

        for price in helpers::gen_dec_range(min_price, max_price) {
            let exists_self = self.payoff.inner.contains_key(&price);
            let exists_rhs = rhs.payoff.inner.contains_key(&price);

            let sum_return = if exists_self && exists_rhs {
                trace!("Price key exists in both options");
                self.payoff.inner.get(&price).unwrap() + rhs.payoff.inner.get(&price).unwrap()
            }
            
            else if exists_self {
                trace!("Price key exists in lhs");
                let rhs_return = rhs.get_price_payoff(&price);
                self.payoff.inner.get(&price).unwrap() + rhs_return
                
            }

            else {
                trace!("Price key exists in rhs");
                let self_return = self.get_price_payoff(&price);
                self_return + rhs.payoff.inner.get(&price).unwrap()
            };

            payoff.inner.insert(price, sum_return);


        }

        OptionCombo::new(vec![self, rhs], payoff)
    }
}

// Basic Option Payoff
impl OptionContract {
    pub fn get_profit_structure(&mut self, mut offset: Option<Decimal>) {
        if offset.is_none() {
            offset = Some(dec!(10))
        };

        // vector of prices for profits to be evaluated at
        let mut evaluation_prices = helpers::gen_offset_range(self.strike, offset.unwrap());

        match self.option_type {
            // Time Value not taken into account
            // Profit levels measured as extrisic value
            // Profit = Market Price - Strike
            OptionType::Call => {
                // for each price, insert profit into map
                evaluation_prices
                    .iter_mut()
                    .map(|eval_price| {
                        // defaults to long call if no side chosen
                        let profit = match self.side.is_some() && self.side.unwrap().is_short() {
                            // short call
                            true => min(self.premium, (self.strike - *eval_price) + self.premium),
                            // long call
                            false => max(-self.premium, (*eval_price - self.strike) - self.premium),
                        };
                        self.payoff.inner.insert(*eval_price, profit);
                    })
                    .count(); // !! Using .count() to force map to eval, better way?
            }
            OptionType::Put => {
                evaluation_prices
                    .iter_mut()
                    .map(|eval_price| {
                        // defaults to long call if no side chosen
                        let profit = match self.side.is_some() && self.side.unwrap().is_short() {
                            // short put
                            true => min(self.premium, (*eval_price - self.strike) + self.premium),
                            // long put
                            false => max(-self.premium, (self.strike - *eval_price) - self.premium),
                        };

                        self.payoff.inner.insert(*eval_price, profit);
                    })
                    .count();
            }
        };
    }

    pub fn get_price_payoff(&self, price: &Decimal) -> Decimal {
        match self.option_type {
            // Time Value not taken into account
            // Profit levels measured as extrisic value
            // Profit = Market Price - Strike
            OptionType::Call => {
                // for each price, insert profit into map

                // defaults to long call if no side chosen
                match self.side.is_some() && self.side.unwrap().is_short() {
                    // short call
                    true => min(self.premium, (self.strike-price) + self.premium),
                    // long call
                    false => max(-self.premium, (price - self.strike) - self.premium),
                }

                // !! Using .count() to force map to eval, better way?
            }
            OptionType::Put => {
                // defaults to long call if no side chosen
                match self.side.is_some() && self.side.unwrap().is_short() {
                    // short put
                    true => min(self.premium, (price - self.strike) + self.premium),
                    // long put
                    false => max(-self.premium, (self.strike - price) - self.premium),
                }
            }
        }
    }
}

