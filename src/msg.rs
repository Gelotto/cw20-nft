use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Uint128};

use crate::models::TradingOrder;

#[cw_serde]
pub struct InstantiateMsg {
  pub token: cw20_base::msg::InstantiateMsg,
  pub price: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
  /// Instantiate CW20 token contract, backed by the NFT's in the vault. This
  /// action can only be performed while the sender owns 100% of the vault. The
  /// default symbol and name of the token is derived from the vault's config.
  CreateToken {
    name: Option<String>,
    symbol: Option<String>,
    price: Uint128,
    supply: u32,
  },
  /// Place one or more market or limit orders. This action can only be
  /// performed once the CW20 token is instantiated.
  PlaceOrders { orders: Vec<TradingOrder> },
  /// Cancel pending orders
  CancelOrders { order_ids: Vec<String> },
  /// Add an NFT to the vault at any time.
  ReceiveNft {
    sender: String,
    token_id: String,
    msg: Binary,
  },
  /// Assuming the sender owns 100% of the cw20 tokens, withdraw the specified
  /// NFT's to the sender or designated recipient's address.
  WithdrawNfts {
    nft_ids: Vec<String>,
    recipient: Option<Addr>,
  },
}

#[cw_serde]
pub enum QueryMsg {
  Select {
    fields: Option<Vec<String>>,
    wallet: Option<Addr>,
  },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct SelectResponse {
  pub owner: Option<Addr>,
}
