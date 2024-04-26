use crate::structs::{profits::PayoffStruct, strategies::*, OptionType, PositionSide};
use chrono::NaiveDate;
use rust_decimal_macros::dec;

pub fn create_mock_option_chain() -> Vec<OptionContract> {
    vec![
        OptionContract {
            side: None,
            option_type: OptionType::Call,
            strike: dec!(100.0),
            premium: dec!(2.5),
            expiration: NaiveDate::from_ymd_opt(2024, 4, 20).unwrap(),
            payoff: PayoffStruct::default(),
            greeks: Greeks::default(),
        },
        OptionContract {
            side: None,
            option_type: OptionType::Call,
            strike: dec!(105.0),
            premium: dec!(1.5),
            expiration: NaiveDate::from_ymd_opt(2024, 4, 20).unwrap(),
            payoff: PayoffStruct::default(),
            greeks: Greeks::default(),
        },
        OptionContract {
            side: None,
            option_type: OptionType::Put,
            strike: dec!(95.0),
            premium: dec!(2.0),
            expiration: NaiveDate::from_ymd_opt(2024, 4, 20).unwrap(),
            payoff: PayoffStruct::default(),
            greeks: Greeks::default(),
        },
        OptionContract {
            side: None,
            option_type: OptionType::Put,
            strike: dec!(90.0),
            premium: dec!(1.0),
            expiration: NaiveDate::from_ymd_opt(2024, 4, 20).unwrap(),
            payoff: PayoffStruct::default(),
            greeks: Greeks::default(),
        },
    ]
}