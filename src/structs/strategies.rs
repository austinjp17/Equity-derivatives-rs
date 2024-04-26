
use std::{cmp::{max, min}, collections::BTreeMap};

use rust_decimal_macros::dec;
use rust_decimal::Decimal;
use chrono::NaiveDate;
use tracing::{info, trace};
use crate::helpers;

use super::{profits::PayoffStruct, OptionType, PositionSide};


#[derive(Default, Debug, Clone)]
pub struct OptionCombo {
    pub legs: Vec<OptionContract>,
    pub payoff: PayoffStruct
}

impl OptionCombo {
    pub fn new(legs: Vec<OptionContract>, payoff: PayoffStruct) -> Self {
        Self {
            legs,
            payoff
        }
    }

    pub fn add_leg(&mut self, leg: OptionContract) {
        assert!(!self.payoff.inner.is_empty());
        assert!(!leg.payoff.inner.is_empty());

        let min_price = min(
            self.payoff.inner.first_key_value().expect("Checked").0,
            leg.payoff.inner.first_key_value().expect("checked").0,
        );

        let max_price = max(
            self.payoff.inner.last_key_value().expect("Checked").0,
            leg.payoff.inner.last_key_value().expect("Checked").0,
        );

        for price in helpers::gen_dec_range(min_price, max_price) {
            let exists_self = self.payoff.inner.contains_key(&price);
            let exists_rhs = leg.payoff.inner.contains_key(&price);

            let sum_return = if exists_self && exists_rhs {
                trace!("Price key exists in both options");
                self.payoff.inner.get(&price).unwrap() + leg.payoff.inner.get(&price).unwrap()
            }
            
            else if exists_self {
                trace!("Price key exists in lhs");
                let rhs_return = leg.get_price_payoff(&price);
                self.payoff.inner.get(&price).unwrap() + rhs_return
                
            }

            else {
                trace!("Price key exists in rhs");
                let self_return: Decimal = self.legs.iter().map(|contract| contract.get_price_payoff(&price)).sum();
                self_return + leg.payoff.inner.get(&price).unwrap()
            };

            self.payoff.inner.insert(price, sum_return);


        }
        self.legs.push(leg);
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Greeks {
    delta: Decimal,
    gamma: Decimal,
    theta: Decimal,
    vega: Decimal
}

/// Basic Call/Put Option Contract
#[derive(Clone, Debug)]
pub struct OptionContract {
    pub side: Option<PositionSide>,
    pub option_type: OptionType,
    pub strike: Decimal,
    pub expiration: NaiveDate,
    pub premium: Decimal,
    pub payoff: PayoffStruct,
    pub greeks: Greeks,
}
impl OptionContract {
    pub fn new(side: Option<PositionSide>, option_type: OptionType, strike: Decimal, expiration: NaiveDate, premium: Decimal, payoff_offset: Option<Decimal>) -> Self {
        let mut contract = Self {
            side,
            option_type,
            strike,
            expiration,
            premium,
            payoff: PayoffStruct::default(),
            greeks: Greeks::default() //TODO
        };
        contract.get_profit_structure(payoff_offset);
        contract
    } 
}


pub struct Spread {
    pub legs: Vec<OptionContract>
}
impl Spread {
    pub fn new(legs: Vec<OptionContract>) -> Self { Self { legs } }
}

pub struct Straddle {
    pub side: Option<PositionSide>,
    pub call_legs: Vec<OptionContract>,
    pub put_legs: Vec<OptionContract>
}

impl Straddle {
    pub fn new(side: Option<PositionSide>, call_legs: Vec<OptionContract>, put_legs: Vec<OptionContract>) -> Self {
        Self {
            side,
            call_legs,
            put_legs,
        }
    }

    // / sum ATM call and put premiums multiplied by 0.85
    // / gives +- expected move
    // pub fn get_implied_move(&self) -> Decimal {
    //     let premium_total = self.call_leg.premium + self.put_leg.premium;
    //     premium_total * dec!(0.85)
    // }
}


pub struct Strangle {
    pub side: Option<PositionSide>,
    pub legs: Vec<OptionContract>
}

impl Strangle { 
    pub fn new(side: Option<PositionSide>, legs: Vec<OptionContract>) -> Self { Self { side, legs } } 
}



