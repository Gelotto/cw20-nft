use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Timestamp, Uint128};

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub enum Side {
  Buy,
  Sell,
}

#[cw_serde]
pub enum TimeInForce {
  FillOrKill,
  ImmediateOrCancel,
  GoodUntilCanceled,
  Time(Timestamp),
}

#[cw_serde]
pub enum TradingOrder {
  Market {
    side: Side,
    quantity: u32,
    time_in_force: Option<TimeInForce>,
  },
  Limit {
    side: Side,
    quantity: u32,
    limit_price: Uint128,
    time_in_force: Option<TimeInForce>,
  },
}
