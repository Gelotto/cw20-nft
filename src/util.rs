use cosmwasm_std::{to_binary, WasmMsg};

use crate::{error::ContractError, msg::InstantiateMsg, state::CW20_CODE_ID};

pub fn build_c20_instantiate_msg(msg: &InstantiateMsg) -> Result<WasmMsg, ContractError> {
  Ok(WasmMsg::Instantiate {
    admin: None,
    code_id: CW20_CODE_ID,
    msg: to_binary(&msg.token)?,
    funds: vec![],
    label: format!("{} NFT CW20 Token", msg.token.name),
  })
}
