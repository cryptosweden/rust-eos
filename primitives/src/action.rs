//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/contracts/eosio/action.hpp#L249-L274>
use crate::{AccountName, Checksum256, ActionName, NumBytes, PermissionLevel, Read, Write, Asset, SerializeData};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use bitcoin_hashes::{
    Hash,
    sha256,
};

impl SerializeData for Action {}
impl SerializeData for  ActionReceipt {}

pub type ActionReceipts = Vec<ActionReceipt>;
pub type PermissionLevels = Vec<PermissionLevel>;

#[derive(Read, Write, NumBytes, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Default)]
#[eosio_core_root_path = "crate"]
pub struct ActionReceipt {
    pub receiver: AccountName,
    pub act_digest: Checksum256,
    pub global_sequence: u64,
    pub recv_sequence: u64,
    pub auth_sequence: AuthSequences,
    pub code_sequence: usize,
    pub abi_sequence: usize,
}

#[derive(Read, Write, NumBytes, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Default)]
#[eosio_core_root_path = "crate"]
pub struct AuthSequence(AccountName, u64);
pub type AuthSequences = Vec<AuthSequence>;

impl AuthSequence {
    pub fn new(
        name: &str,
        number: u64
    ) -> Result<AuthSequence, crate::Error> {
        Ok(
            AuthSequence(
                AccountName::from_str(name)
                    .map_err(|err| crate::Error::from(err) )?,
                number,
            )
        )
    }
}

impl ActionReceipt {

    fn convert_hex_to_checksum256(
        digest_hex: &str
    ) -> Result<Checksum256, crate::Error> {
        let bytes = hex::decode(digest_hex)
            //.map_err(|err| crate::Error::from(err))?;
            .expect("Invalid hex error!"); // TODO: Handle better!
        Self::convert_bytes_to_checksum256(bytes)
    }

    fn convert_bytes_to_checksum256(
        digest_vec: Vec<u8>,
    ) -> Result<Checksum256, crate::Error> {
        let mut digest_arr = [0; 32];
        digest_arr.copy_from_slice(&digest_vec);
        Ok(Checksum256::from(digest_arr))
    }

    pub fn new(
        receiver: &str,
        digest_hex: &str,
        recv_sequence: u64,
        abi_sequence: usize,
        global_sequence: u64,
        code_sequence: usize,
        auth_sequence: AuthSequences,
    ) -> Result<ActionReceipt, crate::Error> {
        Ok(
            ActionReceipt {
                abi_sequence,
                recv_sequence,
                code_sequence,
                auth_sequence,
                global_sequence,
                receiver: AccountName::from_str(receiver)
                    .map_err(|err| crate::Error::from(err))?,
                act_digest: Self::convert_hex_to_checksum256(digest_hex)?,
            }
        )
    }

    pub fn to_digest(&self) -> Vec<u8> {
        sha256::Hash::hash(&self.to_serialize_data()).to_vec()
    }
}

/// This is the packed representation of an action along with meta-data about
/// the authorization levels.
#[derive(Clone, Debug, Serialize, Deserialize, Read, Write, NumBytes, Default)]
#[eosio_core_root_path = "crate"]
pub struct Action {
    /// Name of the account the action is intended for
    pub account: AccountName,
    /// Name of the action
    pub name: ActionName,
    /// List of permissions that authorize this action
    pub authorization: Vec<PermissionLevel>,
    /// Payload data
    pub data: Vec<u8>,
}

impl Action {
    pub fn new(account: AccountName, name: ActionName, authorization: Vec<PermissionLevel>, data: Vec<u8>) -> Self {
        Action { account, name, authorization, data }
    }

    pub fn from_str<T: AsRef<str>, S: SerializeData>(
        account: T,
        name: T,
        authorization: Vec<PermissionLevel>,
        action_data: S
    ) -> Result<Self, crate::Error> {
        let account = AccountName::from_str(account.as_ref()).map_err(|err| crate::Error::from(err) )?;
        let name =  ActionName::from_str(name.as_ref()).map_err(|err| crate::Error::from(err) )?;
        let data = action_data.to_serialize_data();

        Ok(Action { account, name, authorization, data })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Read, Write, NumBytes, Default)]
#[eosio_core_root_path = "crate"]
pub struct ActionTransfer {
    from: AccountName,
    to: AccountName,
    amount: Asset,
    memo: String,
}

impl ActionTransfer {
    pub fn new(from: AccountName, to: AccountName, amount: Asset, memo: String) -> Self {
        ActionTransfer { from, to, amount, memo }
    }

    pub fn from_str<T: AsRef<str>>(from: T, to: T, amount: T, memo: T)
        -> Result<Self, crate::Error>
    {
        let from = AccountName::from_str(from.as_ref()).map_err(|err| crate::Error::from(err) )?;
        let to = AccountName::from_str(to.as_ref()).map_err(|err| crate::Error::from(err) )?;
        let amount = Asset::from_str(amount.as_ref()).map_err(|err| crate::Error::from(err) )?;
        let memo = memo.as_ref().to_string();

        Ok(ActionTransfer { from, to, amount, memo })
    }
}

impl SerializeData for ActionTransfer {}

pub trait ToAction: Write + NumBytes {
    const NAME: u64;

    #[inline]
    fn to_action(
        &self,
        account: AccountName,
        authorization: Vec<PermissionLevel>,
    ) -> Action {
        let mut data = vec![0_u8; self.num_bytes()];
        self.write(&mut data, &mut 0).expect("write");

        Action {
            account,
            name: Self::NAME.into(),
            authorization,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn action_should_work() {
        let permission_level = PermissionLevel::from_str(
            "testa",
            "active"
        ).ok().unwrap();
        let action_transfer = ActionTransfer::from_str(
            "testa",
            "testb",
            "1.0000 EOS",
            "a memo"
        ).ok().unwrap();
        let action = Action::from_str(
            "eosio.token",
            "transfer",
            vec![permission_level],
            action_transfer
        ).ok().unwrap();

        let data = action.to_serialize_data();
        assert_eq!(
            hex::encode(data),
            "00a6823403ea3055000000572d3ccdcd01000000000093b1ca00000000a8ed323227000000000093b1ca000000008093b1ca102700000000000004454f53000000000661206d656d6f"
        );
    }
}
