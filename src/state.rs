use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::Item;

pub const CW20_CODE_ID: u64 = 1;

pub const OWNER: Item<Addr> = Item::new("owner");

pub fn is_owner(
  storage: &dyn Storage,
  addr: &Addr,
) -> StdResult<bool> {
  return Ok(OWNER.load(storage)? == *addr);
}
