use btc_address::BitcoinAddress;
use candid::Principal;
use evm_address::EvmAddress;
use ic_btc_interface::Network;
use ic_solana::types::Pubkey;
use std::str::FromStr;

use crate::error::Error;
use crate::ChainId;
pub mod btc_address;
pub mod evm_address;

//settlement chain
pub const S_BITCOIN_ID: &str = "Bitcoin";
pub const S_BRC20_ID: &str = "Bitcoinbrc20";
pub const S_ICP_ID: &str = "sICP";
pub const S_DOGECOIN_ID: &str = "Dogecoin";
pub const S_SOLANA_ID: &str = "Solana";
pub const S_ETH_ID: &str = "Ethereum";

// execution chain
pub const E_SOLANA_ID: &str = "eSolana";
pub const E_BASE_ID: &str = "Base";
pub const E_ICP_ID: &str = "eICP";

pub fn validate_account(chain_id: &ChainId, address: &String) -> Result<(), Error> {
    match chain_id.as_str() {
        S_BITCOIN_ID | S_BRC20_ID => {
            BitcoinAddress::parse(&address, Network::Mainnet)
                .map_err(|_| Error::InvalidAddress(address.to_owned(), chain_id.to_owned()))?;
        }
        S_ICP_ID | E_ICP_ID => {
            Principal::from_str(&address)
                .map_err(|_| Error::InvalidAddress(address.to_owned(), chain_id.to_owned()))?;
        }
        S_ETH_ID | E_BASE_ID => {
            EvmAddress::from_str(&address)
                .map_err(|_| Error::InvalidAddress(address.to_owned(), chain_id.to_owned()))?;
        }
        S_SOLANA_ID | E_SOLANA_ID => {
            Pubkey::from_str(&address)
                .map_err(|_| Error::InvalidAddress(address.to_owned(), chain_id.to_owned()))?;
        }
        _ => {
            return Err(Error::CustomError(
                "The address type was not recognized!".to_string(),
            ))
        }
    }
    Ok(())
}
